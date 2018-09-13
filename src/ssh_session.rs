use rexpect::spawn;
use rexpect::session::PtyReplSession;
use rexpect::errors::*;

pub trait MyPtyReplSession{
    fn wait_for_prompt(&mut self) -> Result<String> {
        info!("my wait");
        Ok("asdf".to_owned())
    }
}

impl MyPtyReplSession for PtyReplSession {
    fn wait_for_prompt(&mut self) -> Result<String> {
        Ok("asdf".to_owned())
    }
}

fn ssh_session(user: &str, host: &str) -> Result<(PtyReplSession)> {
    let prompt = "[00m ";
    let command = format!("ssh {}@{}", user, host);

    info!("running: {}", command);

    let mut ssh = PtyReplSession {
        // for `echo_on` you need to figure that out by trial and error.
        // For bash and python repl it is false
        echo_on: false,

        // used for `wait_for_prompt()`
        prompt: prompt.to_string(),
        pty_session: spawn(&command, Some(2000))?,
        // command which is sent when the instance of this struct is dropped
        // in the below example this is not needed, but if you don't explicitly
        // exit a REPL then rexpect tries to send a SIGTERM and depending on the repl
        // this does not end the repl and would end up in an error
        quit_command: None,
    };
    ssh.wait_for_prompt()?;
    Ok(ssh)
}

pub fn do_ssh_repl(user: &str, host: &str) -> Result<String> {
    let mut ssh = ssh_session(user, host)?;

    ssh.send_line("ifconfig wlan0")?;
    let (_, mac_line) = ssh.exp_regex("ether [0-9a-f:]+")?;
    ssh.wait_for_prompt()?;

    ssh.send_line("ifconfig wlan0")?;
    let (_, mac_line) = ssh.exp_regex("ether [0-9a-f:]+")?;
    ssh.wait_for_prompt()?;

    ssh.send_line("ifconfig wlan0")?;
    let (_, mac_line) = ssh.exp_regex("ether [0-9a-f:]+")?;
    ssh.wait_for_prompt()?;



    let mac = mac_line.split(" ").nth(1).unwrap();
    println!("mac ={}", mac);
//    ssh.wait_for_prompt()?;
//    ssh.send_line(",l")?;
//    ssh.exp_string("ed is the best editor evar$")?;
//    ssh.send_line("Q")?;
//    ssh.exp_eof()?;
    Ok(mac.to_owned())
}