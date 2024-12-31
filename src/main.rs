/*
 * @Author: jack cymqqqq@gmail.com
 * @Date: 2024-12-29 17:48:20
 * @LastEditors: jack cymqqqq@gmail.com
 * @LastEditTime: 2024-12-31 16:54:43
 * @FilePath: /crypto-agent/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

use std::str::FromStr;

use aluvm::{CoreConfig, LibSite};
use amplify::confinement::{SmallString, TinyString};
use amplify::num::u256;
use commit_verify::{Digest, Sha256};
use hypersonic::embedded::{EmbeddedArithm, EmbeddedImmutable, EmbeddedProc, EmbeddedReaders};
use hypersonic::{
    Api, ApiInner, AppendApi, AuthToken, Codex, CodexId, DestructibleApi, Identity, Schema, Stock, FIELD_ORDER_SECP
};
use rig::completion::*;
use rig::providers::openai::{self, Client};

const KEY: &str="";

mod libs {
    use aluvm::{aluasm, Lib};

    pub fn success() -> Lib {
        let code = aluasm! {
            stop;
        };
        Lib::assemble(&code).unwrap()
    }
}

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_types;

mod stl {
    use amplify::confinement::SmallString;
    use strict_encoding::{StrictDecode, StrictDumb, StrictEncode};
    use strict_types::{
        libname, stl::std_stl, LibBuilder, SemId, StrictType, SymbolicSys, SystemBuilder, TypeLib,
        TypeSystem,
    };

    pub const LIB_NAME_AGENT: &str = "AGENT";

    #[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    // #[display(r#"{name}"#)]
    #[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
    #[strict_type(lib = LIB_NAME_AGENT)]
    pub struct PlantName(SmallString);

    #[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    // #[display(r#"{description}"#)]
    #[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
    #[strict_type(lib = LIB_NAME_AGENT)]
    pub struct Plant(SmallString);

    #[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    // #[display(r#"{description}"#)]
    #[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
    #[strict_type(lib = LIB_NAME_AGENT)]
    pub struct PlantOwner(SmallString);

    
    pub fn stl() -> TypeLib {
        LibBuilder::new(
            libname!(LIB_NAME_AGENT),
            tiny_bset! {
                std_stl().to_dependency(),
            },
        )
        .transpile::<Plant>()
        .transpile::<PlantName>()
        .transpile::<PlantOwner>()
        .compile()
        .expect("invalid agent type library")
    }

    #[derive(Debug)]
    pub struct AgentTypes(SymbolicSys);

    impl Default for AgentTypes {
        fn default() -> Self {
            AgentTypes::new()
        }
    }

    impl AgentTypes {
        pub fn new() -> Self {
            Self(
                SystemBuilder::new()
                    .import(std_stl())
                    .unwrap()
                    .import(stl())
                    .unwrap()
                    .finalize()
                    .unwrap(),
            )
        }

        pub fn type_system(&self) -> TypeSystem {
            let types = stl().types;
            let types = types.iter().map(|(tn, ty)| ty.sem_id_named(tn));
            self.0.as_types().extract(types).unwrap()
        }

        pub fn get(&self, name: &'static str) -> SemId {
            *self.0.resolve(name).unwrap_or_else(|| {
                panic!("type '{name}' is absent in standard RGBContract type library")
            })
        }
    }
}

fn codex(agnet_name: &str) -> Codex {
    let lib = libs::success();
    let lib_id = lib.lib_id();

    Codex {
        name: TinyString::from_str(agnet_name).unwrap(),
        developer: Identity::default(),
        version: default!(),
        timestamp: 1732529307,
        field_order: FIELD_ORDER_SECP,
        input_config: CoreConfig::default(),
        verification_config: CoreConfig::default(),
        verifiers: tiny_bmap! {
            0 => LibSite::new(lib_id, 0)
        },
        reserved: default!(),
    }
}

fn api(codex_id: CodexId) -> Api {
    let types = stl::AgentTypes::new();

    Api::Embedded(ApiInner::<EmbeddedProc> {
        version: default!(),
        codex_id,
        timestamp: 1732529307,
        name: None,
        developer: Identity::default(),
        append_only: tiny_bmap! {
            vname!("_plants") => AppendApi {
                sem_id: types.get("AGENT.PlantName"),
                raw_sem_id: types.get("AGENT.Plant"),
                published: true,
                adaptor: EmbeddedImmutable(u256::ZERO),
            },
        },
        destructible: tiny_bmap! {
            vname!("agentOwners") => DestructibleApi {
                sem_id: types.get("AGENT.PlantOwner"),
                arithmetics: EmbeddedArithm::NonFungible,
                adaptor: EmbeddedImmutable(u256::ZERO),
            }
        },
        readers: tiny_bmap! {
            vname!("plants") => EmbeddedReaders::MapV2U(vname!("_plants")),
        },
        verifiers: tiny_bmap! {
            vname!("setup") => 0,
        },
        errors: Default::default(),
    })
}

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
            let stock = Stock::new(articles, "data");
            
        }
        ModelChoice::ToolCall(tool_name, tool_params) => {
            // Handle the completion response as a tool call
            println!("Received tool call: {} {:?}", tool_name, tool_params);
        }
    }
}
