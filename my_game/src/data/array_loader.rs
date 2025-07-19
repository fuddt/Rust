use ndarray::Array2;
use serde_json::Value;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ArrayLoaderError {
    JsonParseError(serde_json::Error),
    InvalidFormat(String),
}

impl fmt::Display for ArrayLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArrayLoaderError::JsonParseError(err) => write!(f, "JSON parse error: {}", err),
            ArrayLoaderError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

impl Error for ArrayLoaderError {}

impl From<serde_json::Error> for ArrayLoaderError {
    fn from(err: serde_json::Error) -> Self {
        ArrayLoaderError::JsonParseError(err)
    }
}

/// JSONデータから2次元配列を読み込み、ndarrayを作成する
pub fn load_array_from_json(json_str: &str) -> Result<Array2<i32>, ArrayLoaderError> {
    let json_value: Value = serde_json::from_str(json_str)?;
    
    // JSONが配列であることを確認
    let array = json_value.as_array()
        .ok_or_else(|| ArrayLoaderError::InvalidFormat("Root element must be an array".to_string()))?;
    
    if array.is_empty() {
        return Err(ArrayLoaderError::InvalidFormat("Array cannot be empty".to_string()));
    }
    
    // 各行が配列であることを確認し、サイズを取得
    let rows = array.len();
    let cols = array[0].as_array()
        .ok_or_else(|| ArrayLoaderError::InvalidFormat("Each row must be an array".to_string()))?
        .len();
    
    if cols == 0 {
        return Err(ArrayLoaderError::InvalidFormat("Rows cannot be empty".to_string()));
    }
    
    // データを収集
    let mut data = Vec::with_capacity(rows * cols);
    
    for (row_idx, row) in array.iter().enumerate() {
        let row_array = row.as_array()
            .ok_or_else(|| ArrayLoaderError::InvalidFormat(format!("Row {} is not an array", row_idx)))?;
        
        if row_array.len() != cols {
            return Err(ArrayLoaderError::InvalidFormat(
                format!("Row {} has {} columns, expected {}", row_idx, row_array.len(), cols)
            ));
        }
        
        for (col_idx, cell) in row_array.iter().enumerate() {
            let value = cell.as_i64()
                .ok_or_else(|| ArrayLoaderError::InvalidFormat(
                    format!("Cell at ({}, {}) is not an integer", row_idx, col_idx)
                ))?;
            
            data.push(value as i32);
        }
    }
    
    // ndarrayを作成
    Array2::from_shape_vec((rows, cols), data)
        .map_err(|_| ArrayLoaderError::InvalidFormat("Failed to create ndarray".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_simple_array() {
        let json = r#"[[0, 1], [1, 0]]"#;
        let array = load_array_from_json(json).unwrap();
        
        assert_eq!(array.shape(), &[2, 2]);
        assert_eq!(array[[0, 0]], 0);
        assert_eq!(array[[0, 1]], 1);
        assert_eq!(array[[1, 0]], 1);
        assert_eq!(array[[1, 1]], 0);
    }

    #[test]
    fn test_invalid_json() {
        let json = "invalid json";
        assert!(load_array_from_json(json).is_err());
    }

    #[test]
    fn test_inconsistent_row_length() {
        let json = r#"[[0, 1], [1]]"#;
        assert!(load_array_from_json(json).is_err());
    }
}
