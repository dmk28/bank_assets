enum Assets {
    Stocks {
        qtd: i32,
        value: f32,
        rating: f32
    },
    Bonds {
        qtd: i32,
        value: f32,
        interest: f32,
    },
    Funds(f32),
    Cash(f32)
}

impl Assets {

    fn feedback_stocks(&self) -> (i32, f32) {
        if let Assets::Stocks {qtd, value, rating} = self {
            (*qtd, *value)
        } else {
            panic!("Not a stocks asset");
        }
    }

    fn feedback_bonds(&self) -> i32 {
        if let Assets::Bonds {qtd, ..} = self {
            *qtd
        } else {
            panic!("Not a bond asset");
        }
    }

    fn feedback_funds_and_cash(&self) -> (f32, f32) {
        match self {
            Assets::Funds(funds) => {
                match self {
                    Assets::Cash(cash) => (*funds, *cash),
                    _ => panic!("No cash asset found")
                }
            },
            Assets::Cash(cash) => {
                match self {
                    Assets::Funds(funds) => (*funds, *cash),
                    _=> panic!("No funds assets found")
                }
            },
            _ => panic!("No monetary assets found")
        }
    }

    fn calculate_stocks_after_ratings(&self) -> f32 {
        if let Assets::Stocks {qtd, value, rating} = self {
            (*qtd as f32) * (*value as f32) / (*rating as f32)
        } else {
            panic!("Not a stock asset");
        }
    }

    fn calculate_bonds_value(&self) -> f32 {
        if let Assets::Bonds {qtd, value, interest} = self {
            (*qtd as f32 * *value as f32) * (1.0 + *interest as f32)
        } else {
            panic!("Not a bond asset");
        }
    }

    fn value(&self) -> f32 {
        match self {
            Assets::Stocks { qtd, value, rating } => (*qtd as f32) * (*value as f32) / (*rating as f32),
            Assets::Bonds { qtd, value, interest } => (*qtd as f32) * (*value as f32) * (1.0 + *interest as f32),
            Assets::Funds(amount) => *amount,
            Assets::Cash(amount) => *amount,
        }
    }
}

fn main() {
    let assets = vec![
        Assets::Stocks {qtd: 100, value: 200.0, rating: 23.5},
        Assets::Bonds {qtd: 200,  value: 1000.0, interest: 0.015 },
        Assets::Cash(3000.0),
        Assets::Funds(1500.0),
    ];

   
    

    let total_value: f32 = assets.iter().map(|asset| asset.value()).sum();

    println!("Total value {}", total_value);
}
