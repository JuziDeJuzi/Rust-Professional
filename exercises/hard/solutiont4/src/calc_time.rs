pub fn time_info(time: &str) -> String {
    let parts: Vec<&str> = time.split('-').collect();
    let year = parts[0].parse::<i32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();
    let day = parts[2].parse::<u32>().unwrap();

    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    // 计算当天是本年的第几天
    let mut day_of_year = day;
    for m in 1..month {
        day_of_year += days_in_month[(m as usize) - 1];
    }

    // 计算周几（1为周一，7为周日）
    // 2025年1月1日是周三（3）
    let mut weekday = (2 + day_of_year as i32) % 7;
    if weekday == 0 {
        weekday = 7;
    }

    // 计算周数（1月1日为第一周，1月6日为第二周开始，最后6天为2026年第一周）
    let week_number = if day_of_year <= 5 {
        1 // 第一周：1月1日到1月5日
    } else if day_of_year >= 365 - 6 { // 最后6天（12月26日起）为2026年第一周
        1 // 2026年的第一周
    } else {
        1 + (day_of_year - 6) / 7 + 1 // 从1月6日开始，每7天为一周
    };

    // 计算当年剩余天数
    let days_left = 365 - day_of_year;

    // 计算距离下一个正月初一的天数
    let cny_2025 = 29; // 2025-01-29
    let cny_2026 = 365 + 31 + 17; // 2026-02-17，从2025-01-01起第413天
    let days_to_cny = if day_of_year < cny_2025 {
        cny_2025 - day_of_year // 到2025年春节
    } else {
        cny_2026 - day_of_year // 到2026年春节
    };

    // 计算距离下次A股开盘天数
    let holidays = vec![
        "2025-01-01", "2025-01-28", "2025-01-29", "2025-01-30", "2025-01-31",
        "2025-02-01", "2025-02-02", "2025-02-03", "2025-02-04",
        "2025-04-04", "2025-04-05", "2025-04-06",
        "2025-05-01", "2025-05-02", "2025-05-03", "2025-05-04", "2025-05-05",
        "2025-05-31", "2025-06-01", "2025-06-02",
        "2025-10-01", "2025-10-02", "2025-10-03", "2025-10-04", "2025-10-05",
        "2025-10-06", "2025-10-07", "2025-10-08",
        "2026-01-01",
    ];
    let mut current_date = day_of_year as i32;
    let mut days_to_open = 0;
    loop {
        current_date += 1;
        days_to_open += 1;
        let (current_year, adjusted_date) = if current_date > 365 {
            (2026, current_date - 365) // 跨到2026年
        } else {
            (2025, current_date) // 仍在2025年
        };
        let current_weekday = (2 + current_date) % 7;
        let current_weekday = if current_weekday == 0 { 7 } else { current_weekday };
        if current_weekday == 6 || current_weekday == 7 {
            continue; // 周六或周日
        }
        // 计算当前月份和日期
        let mut accumulated_days = 0;
        let mut current_month = 1;
        for i in 0..12 {
            if accumulated_days + days_in_month[i] >= adjusted_date as u32 {
                current_month = i + 1;
                break;
            }
            accumulated_days += days_in_month[i];
        }
        let current_day = adjusted_date - accumulated_days as i32;
        let date_str = format!("{}-{:02}-{:02}", current_year, current_month, current_day);
        if holidays.contains(&date_str.as_str()) {
            continue;
        }
        break;
    }

    format!(
        "{},{},{},{},{},{}",
        week_number,
        weekday,
        day_of_year,
        days_left,
        days_to_cny,
        days_to_open-1
    )
}
