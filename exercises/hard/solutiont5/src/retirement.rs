use chrono::{NaiveDate, Datelike, Month};

fn add_months(date: NaiveDate, months: i32) -> NaiveDate {
    let (year, month) = {
        let month = date.month0() as i32 + months;
        let year = date.year() + (month / 12);
        let month = (month % 12) + 1;
        (year, month)
    };
    NaiveDate::from_ymd_opt(year, month as u32, date.day()).unwrap()
}

struct TypeParams {
    base_age: i32,
    threshold_year: i32,
    period_length: i32,
    max_delay: i32,
}

fn get_type_params(tp: &str) -> TypeParams {
    match tp {
        "男职工" => TypeParams { base_age: 60, threshold_year: 1965, period_length: 4, max_delay: 36 },
        "原法定退休年龄55周岁女职工" => TypeParams { base_age: 55, threshold_year: 1970, period_length: 4, max_delay: 36 },
        "原法定退休年龄50周岁女职工" => TypeParams { base_age: 50, threshold_year: 1975, period_length: 2, max_delay: 60 },
        _ => panic!("未知人员类型"),
    }
}

pub fn retire_time(time: &str, tp: &str) -> String {
    let birth_date = NaiveDate::parse_from_str(&format!("{}-01", time), "%Y-%m-%d").unwrap();
    let birth_year = birth_date.year();
    let birth_month = birth_date.month();

    // 获取类型参数
    let params = get_type_params(tp);
    let base_age = params.base_age;
    let threshold_year = params.threshold_year;
    let period_length = params.period_length;
    let max_delay = params.max_delay;

    // 计算延迟月数
    let delay_months = if birth_year < threshold_year {
        0
    } else {
        let periods_per_year = 12 / period_length;
        let period_number = (birth_year - threshold_year) as i64 * periods_per_year as i64
            + ((birth_month as i64 - 1) / period_length as i64) + 1;
        std::cmp::min(period_number as i32, max_delay)
    };

    // 计算退休日期
    let mut retirement_date = birth_date;
    retirement_date = add_months(retirement_date, base_age * 12);
    retirement_date = add_months(retirement_date, delay_months);

    // 计算退休年龄（基础年龄 + 延迟月数/12）
    let retirement_age = base_age as f64 + (delay_months as f64 / 12.0);

    format!("{}-{:02},{},{}",
            retirement_date.year(),
            retirement_date.month(),
            format_result(retirement_age),
            delay_months)
}

fn format_result(a :f64) -> String {
    if (a * 100.0).round() % 100.0 == 0.0 {
        return format!("{}", a as u64);
    } else {
        return format!("{:.2}", a);
    }
}