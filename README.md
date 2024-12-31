<!--
 * @Author: jack cymqqqq@gmail.com
 * @Date: 2024-12-29 17:47:07
 * @LastEditors: jack cymqqqq@gmail.com
 * @LastEditTime: 2024-12-31 16:54:15
 * @FilePath: /crypto-agent/README.md
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
-->
# crypto-agent

This is an exploration of combining the RGB protocol with AI Agents, and it's also a thought experiment. Although it's only in the preliminary stage, continuous research will be conducted in the future.

It's a RGB crypto agent combined with the Rig framework(an Ai Agent library) and RGB protocol.
You can easily copy your openai api key in the source code, and use gpt-4o model(current stage).

# Fundamental

1.Obtain the response from openai api(or your huggingface model, but now the openai is only supported).

2.Design a RGB crypto schema, define a mojo name as your contract name, and then input the name into the ai model to get the response. Finally, the result is written to the RGB contract.


# Libs 
1.Rig framework
2.Based on rgb 0.12.beta.4 (Because it's easy to design experimental contract in current stage)

# Quick Start
Make sure you have installed the rust environment
Just exec `cargo r`

# Development

In this example, we show the `plant agent`, it means you can input your prompt to get a response, then write it into the RGB contract.
```rust


#[tokio::main]
pub async fn main() {
    // Initialize the OpenAI client and a completion model
    let openai = Client::new(KEY);

    let gpt_4 = openai.completion_model(openai::GPT_4);
    let agnet_name = "PlantAgent";
    let plant_name = "tomato";
    // Create the completion request
    let request = gpt_4.completion_request(&format!(
        "I give you a plant name '{}', then tell me something about it within 30 words, print its english and chinese. When a sentence ends, then switch to the next line
",
        plant_name
    ))
    .preamble("You are a plant professor, an extremely smart and know everything about plant.".to_string())
    .temperature(0.5)
    .build();
    // setup your RGB agent schema
    let agent_codex = codex(agnet_name);
    let agent_api = api(agent_codex.codex_id());

    let types = stl::AgentTypes::new();
    let issuer = Schema::new(
        agent_codex,
        agent_api,
        [libs::success()],
        types.type_system(),
    );
    // save your contract
    issuer
        .save("Agent.issuer")
        .expect("unable to save issuer to a file");
    let seed = &[0xCA; 30][..];
    let mut auth = Sha256::digest(&seed);
    let mut next_auth = || -> AuthToken {
        auth = Sha256::digest(&*auth);
        let mut buf = [0u8; 30];
        buf.copy_from_slice(&auth[..30]);
        AuthToken::from(buf)
    };

    // Send the completion request and get the completion response
    let response = gpt_4
        .completion(request)
        .await
        .expect("Failed to get completion response");

    // Handle the completion response
    match response.choice {
        ModelChoice::Message(message) => {
            // Handle the completion response as a message
            println!("Received message: {}", message);
            let agent_auth = next_auth();
            let articles = issuer
                .start_issue("setup")
                // Jack
                .append("_plants", svstr!(plant_name), Some(svstr!(message)))
                .assign("agentOwners", agent_auth, svstr!("Jack Choi"), None)
                .finish::<0>("PlantAgent", 1732529307);
            println!("id {:?}", &articles.contract_id().to_string());
            let mut stock = Stock::new(articles, "data");
            
        }
        ModelChoice::ToolCall(tool_name, tool_params) => {
            // Handle the completion response as a tool call
            println!("Received tool call: {} {:?}", tool_name, tool_params);
        }
    }
}

```


# FAQ
If there are any questions, you can open an new issue to describe it.