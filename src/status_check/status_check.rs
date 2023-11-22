use reqwest;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = "https://api.github.com/repos/andyyuyc/Jersey/commits";
    let client = reqwest::Client::new();

    let res = client.get(repo)
        .header("User-Agent", "request")
        .send()
        .await?;

    let commits: Value = res.json().await?;

    // 遍历 JSON 数组并提取需要的字段
    if let Value::Array(commits_array) = commits {
        for commit in commits_array {
            let committer = &commit["commit"]["committer"];
            let time = committer["date"].as_str().unwrap_or("");
            let name = committer["name"].as_str().unwrap_or("");
            let events_url = commit["committer"]["events_url"].as_str().unwrap_or("");
            let sha = commit["sha"].as_str().unwrap_or("");
            let url = commit["url"].as_str().unwrap_or("");

            println!("Time: {},\nName: {},\nEvents URL: {},\nSHA: {},\nURL: {},\n\n", time, name, events_url, sha, url);
        }
    }

    Ok(())
}




/*
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_main_function() {
        let result = main();
        assert!(result.is_ok());
    }
}
*/
