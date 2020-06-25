use clap::{ ArgMatches, SubCommand, App, };
use secp256k1::{ Secp256k1, Message, SecretKey, PublicKey, };
use crate::util::XResult;
use crate::cmd::Command;

pub struct CommandTest {
}

impl Command for CommandTest {

    fn name(&self) -> &str { "test" }

    fn subcommand<'a>(&self) -> Option<App<'a, 'a>> {
        Some(SubCommand::with_name(self.name()).about("test"))
    }

    fn run(&self, _arg_matches: &ArgMatches, _sub_arg_matches: &ArgMatches) -> XResult<()> {
        println!("This is test command!");

        let secp = Secp256k1::new();
        let secret_key_bytes = hex::decode("bc60c7fcae1a43947cdff39f9a5b8812025809160cc7f69a73027bae7c105a71")?;
        let secret_key = SecretKey::from_slice(&secret_key_bytes[..])?;
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        println!("{}", secret_key);
        println!("{}", hex::encode(&public_key.serialize_uncompressed()[..]));

        let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");
        let k = crate::p256k1::P256k1SecretKey::from_hex("bc60c7fcae1a43947cdff39f9a5b8812025809160cc7f69a73027bae7c105a71")?;
        let sig = k.sign(&message);
        println!("SIG: {}", sig);
        let sig_b64 = base64::encode(&sig.serialize_der());
        println!("SIG: {}", sig_b64);

        let r = secp.verify(&message, &sig, &public_key).is_ok();
        println!("RESULT: {}", r);

        println!("\n{}\n", ".".repeat(76));
        let r = crate::signed_message_parser::DidSignedMessageBuilder::new_from_str("hello world")
            .key_id("did:ccp:3nBPSZU1q6mmxha5Jbg8NcRNGGNt#key-1")
            // .hash(&("SHA256-".to_owned() + &hex::encode(&secret_key_bytes)))
            .hash(&("SHA256-".to_owned() + &base64::encode(&secret_key_bytes)))
            .signature(&hex::decode(&format!("{}", sig))?)
            .build()?;
        println!("{}", r.as_string());

        println!("\n{}\n", ".".repeat(76));
        let r = crate::signed_message_parser::DidSignedMessageBuilder::new_from_bytes(
            (1..100).map(|i| format!("{}", i)).collect::<Vec<_>>().join("").as_bytes()
        )
            .key_id("did:ccp:3nBPSZU1q6mmxha5Jbg8NcRNGGNt#key-1")
            .hash(&("SHA256-".to_owned() + &hex::encode(&secret_key_bytes)))
            // .hash(&("SHA256-".to_owned() + &base64::encode(&secret_key_bytes)))
            .signature(&hex::decode(&format!("{}", sig))?)
            .build()?;
        println!("{}", r.as_string());

        Ok(())
    }
}