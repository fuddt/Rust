// 異常値の検出
// data.csvには９５％のデータが0〜1の範囲の値を持つデータが格納されている
// 残りの5%のデータには5を入れてある
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;


// # OutlierDetector構造体
//
//このDocstringではmarkdownが使える
// フィールドについて
// - data: Vec<f64>型
//   - データを格納するベクタ

struct OutlierDetector {
    data: Vec<f64>,
}

impl OutlierDetector {
    // 新しいインスタンスを作成
    fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file: File = File::open(file_path)?;
        let mut reader: csv::Reader<File> = ReaderBuilder::new().has_headers(false).from_reader(file);

        let mut data: Vec<f64> = Vec::new();
        for result in reader.records() {
            let record: csv::StringRecord = result?;
            let value: f64 = record[0].parse()?;
            data.push(value);
        }

        Ok(OutlierDetector { data })
    }

    // 平均を計算
    fn mean(&self) -> f64 {
        self.data.iter().sum::<f64>() / self.data.len() as f64
    }

    // 分散を計算
    fn variance(&self, mean: f64) -> f64 {
        self.data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / self.data.len() as f64
    }

    // 信頼区間の閾値を計算
    fn threshold(&self, z_score: f64) -> f64 {
        let mean: f64 = self.mean();
        let variance: f64 = self.variance(mean);
        let std_dev: f64 = variance.sqrt();
        mean + z_score * std_dev
    }

    // 異常値を検出
    fn detect_outliers(&self, threshold: f64) -> Vec<f64> {
        self.data.iter().cloned().filter(|&x| x > threshold).collect()
    }

    // 結果を表示
    fn display_results(&self, threshold: f64) {
        println!("平均: {:.6}", self.mean());
        println!("閾値: {:.6}", threshold);
        println!("異常値:");
        for outlier in self.detect_outliers(threshold) {
            println!("{:.6}", outlier);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // CSVファイルのパス
    let file_path: &str = "data.csv";

    // OutlierDetectorのインスタンスを作成
    let detector: OutlierDetector = OutlierDetector::new(file_path)?;

    // 信頼区間95%（片側）
    let z_score: f64 = 1.645; // 片側
    let threshold: f64 = detector.threshold(z_score);

    // 結果を表示
    detector.display_results(threshold);

    Ok(())
}
