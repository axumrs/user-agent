use std::sync::LazyLock;

/// 桌面UA列表
static DESKTOP_USER_AGENTS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        // -- DEBIAN 12
        "Mozilla/5.0 (X11; Linux x86_64; rv:140.0) Gecko/20100101 Firefox/140.0", // FIREFOX v140.4.0esr
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36", // CHROME v142.0.7444.59
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36 OPR/123.0.0.0", // OPERA v123.0.5669.23
        // -- DEBIAN 13
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36", // CHROME 147.0.7727.137
        "Mozilla/5.0 (X11; Linux x86_64; rv:140.0) Gecko/20100101 Firefox/140.0", // FIREFOX 140.10.1esr
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36 OPR/131.0.0.0", // OPERA 131.0.5877.5
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36 Brave/1.89.145", // BRAVE 1.89.145
        // -- WIN 11
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:150.0) Gecko/20100101 Firefox/150.0", // FIREFOX 150.0.1
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36", // CHROME 147.0.7727.138
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36 OPR/130.0.0.0", // OPERA 130.0.5847.92
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36 Avast/134.0.0.0", // AVAST 134.0.29548.179
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36 Vivaldi/7.9.3970.60", // Vivaldi 7.9.3970.60
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Iron Safari/537.36", // Iron v115.0.5850.0,
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36 Edg/147.0.3912.98", // EDGE 147.0.3912.98
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36 BRAVE/1.89.145", // BRAVE 1.89.145
        // == MAC OS, 由 GEMINI 生成 ==
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.1 Safari/605.1.15",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 14.0; rv:150.0) Gecko/20100101 Firefox/150.0",
    ]
});

/// 移动UA列表
static MOBILE_USER_AGENTS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        // -- iphone
        "Mozilla/5.0 (iPhone; CPU iPhone OS 18_6_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/143 Mobile/15E148 Version/17.7", // FIREFOX
        "Mozilla/5.0 (iPhone; CPU iPhone OS 18_6_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/141.0.7390.96 Mobile/15E148 Safari/604.1", // CHROME
        "Mozilla/5.0 (iPhone; CPU iPhone OS 18_6_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.6 Mobile/15E148 Safari/604.1", // SAFARI
        // == GEMINI 生成的UA ==
        "Mozilla/5.0 (iPhone; CPU iPhone OS 19_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/19.0 Mobile/15E148 Safari/604.1", // iPhone 17 Pro - Safari
        "Mozilla/5.0 (iPhone; CPU iPhone OS 19_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/145.0.7123.98 Mobile/15E148 Safari/604.1", // iPhone 16 - Chrome
        "Mozilla/5.0 (Linux; Android 16; Pura 80 Ultra) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.7123.102 Mobile Safari/537.36 HuaweiBrowser/16.0.2.310", // 华为 Pura 80 Ultra - 华为浏览器
        "Mozilla/5.0 (Linux; Android 16; SM-S9480) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.7123.102 Mobile Safari/537.36", // 三星 Galaxy S26 Ultra - Chrome
        "Mozilla/5.0 (Linux; Android 16; Mi 16 Pro; Build/UKQ1.231207.002) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.7123.102 Mobile Safari/537.36 XiaoMi/MiuiBrowser/19.2.5", // 小米 16 Pro - 小米浏览器
    ]
});

/// 所有UA列表
static ALL_USER_AGENTS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    let mut v = Vec::with_capacity(DESKTOP_USER_AGENTS.len() + MOBILE_USER_AGENTS.len());
    v.extend_from_slice(&DESKTOP_USER_AGENTS);
    v.extend_from_slice(&MOBILE_USER_AGENTS);
    v
});

/// 获取领域
pub enum Domain {
    /// 桌面端
    Desktop,
    /// 移动端
    Mobile,
    /// 所有
    All,
}

/// 桌面端 User-Agent 列表
pub fn desktop_user_agents() -> &'static [&'static str] {
    &DESKTOP_USER_AGENTS
}

/// 移动端 User-Agent 列表
pub fn mobile_user_agents() -> &'static [&'static str] {
    &MOBILE_USER_AGENTS
}

/// 所有 User-Agent 列表
pub fn all_user_agents() -> &'static [&'static str] {
    &ALL_USER_AGENTS
}

/// 通过指定领域随机获取一个 User-Agent
#[cfg(feature = "random")]
fn user_agent_with_domain(domain: Domain) -> &'static str {
    let uas = match domain {
        Domain::All => all_user_agents(),
        Domain::Desktop => desktop_user_agents(),
        Domain::Mobile => mobile_user_agents(),
    };

    let idx = rand::random_range(0..uas.len());
    uas[idx]
}

/// 随机获取一个桌面端 User-Agent
#[cfg(feature = "random")]
pub fn desktop_user_agent() -> &'static str {
    user_agent_with_domain(Domain::Desktop)
}

/// 随机获取一个移动端 User-Agent
#[cfg(feature = "random")]
pub fn mobile_user_agent() -> &'static str {
    user_agent_with_domain(Domain::Mobile)
}

/// 随机获取一个 User-Agent，不区分桌面端或移动端
#[cfg(feature = "random")]
pub fn user_agent() -> &'static str {
    user_agent_with_domain(Domain::All)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_agent() {
        println!("desktop: {}", desktop_user_agent());
        println!("mobile: {}", mobile_user_agent());
        println!("all: {}", user_agent());
    }
    #[test]
    fn test_user_agents() {
        println!("desktop: {:?}", desktop_user_agents());
        println!("mobile: {:?}", mobile_user_agents());
        println!("all: {:?}", all_user_agents());
    }
}
