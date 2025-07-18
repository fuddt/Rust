use rand::Rng;

enum GachaResult {
    Hit(u32),  // 当たり金額
    Miss,      // ハズレ（メッセージは処理側で決める）
}

fn gacha() -> GachaResult {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(0..10);

    if roll == 0 || roll == 1 || roll == 2 {
        GachaResult::Hit(500)
    } else {
        GachaResult::Miss
    }
}

fn open_gacha(result: GachaResult) {
    match result {
        GachaResult::Hit(amount) => {
            println!("当たり！{}円を獲得！", amount);
        }
        GachaResult::Miss => {
            println!("ハズレ...また回してね！");
        }
    }
}

fn main() {
    for i in 1..=5 {
        println!("--- {}回目のガチャ ---", i);
        let result = gacha();
        open_gacha(result);
    }
}