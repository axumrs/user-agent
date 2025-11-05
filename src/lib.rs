use lazy_static::lazy_static;

lazy_static! {
    /// 桌面UA列表
    static ref DESKTOP_USER_AGENTS: Vec<&'static str> = vec![
        // -- DEBIAN 12
        "Mozilla/5.0 (X11; Linux x86_64; rv:140.0) Gecko/20100101 Firefox/140.0", // FIREFOX v140.4.0esr
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36", // CHROME v142.0.7444.59
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36 OPR/123.0.0.0", // OPERA v123.0.5669.23

        // -- WIN 11
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:144.0) Gecko/20100101 Firefox/144.0", // FIREFOX v144.0.2
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36", // CHROME v142.0.7444.60
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36 OPR/122.0.0.0", // OPERA v122.0.5643.142
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36 Avast/134.0.0.0", // AVAST v134.0.29548.179
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36", // Vivaldi v7.6.3797.63
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Iron Safari/537.36", // Iron v115.0.5850.0

    ];

    /// 移动UA列表
    static ref MOBILE_USER_AGENTS : Vec<&'static str> = vec![
        // -- iphone
        "Mozilla/5.0 (iPhone; CPU iPhone OS 18_6_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/143 Mobile/15E148 Version/17.7", // FIREFOX
        "Mozilla/5.0 (iPhone; CPU iPhone OS 18_6_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/141.0.7390.96 Mobile/15E148 Safari/604.1", // CHROME
        "Mozilla/5.0 (iPhone; CPU iPhone OS 18_6_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.6 Mobile/15E148 Safari/604.1", // SAFARI
    ];

    /// 所有UA列表
    static ref ALL_USER_AGENTS: Vec<&'static str> = {
        let mut v = Vec::with_capacity(DESKTOP_USER_AGENTS.len() + MOBILE_USER_AGENTS.len());
        v.extend_from_slice(&DESKTOP_USER_AGENTS);
        v.extend_from_slice(&MOBILE_USER_AGENTS);
        v
    };
}

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
