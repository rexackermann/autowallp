use std::time::Duration;
use std::thread::sleep;

fn main() {

        let wn : i64 = 100000000000000000;
        println!("{:?}", wn);

    while 1==1 {
        
        sleep(Duration::from_millis(200));
        println!("Hello, world!");

        use std::process::Command;
        let w = Command::new("xdotool")
                        .arg("get_desktop")
                        .output()
                        .expect("xdotool command failed to start");
        let wl = w.stdout;

        let mut wo = String::from_utf8(wl).expect("Found invalid UTF-8");
        let wo : i64 = wo.trim().parse().unwrap();
        println!("{:?}", wo);


        if wo != wn {
            if wo == 0 {
                println!("Workspace 0 \n Wall0");
                Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri $HOME/.config/autowallp/wall-light0")
                         .output();
                         Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri-dark $HOME/.config/autowallp/wall-dark0")
                         .output();
            } else if wo == 1 {
                println!("Workspace 1 \n Wall1");
                Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri $HOME/.config/autowallp/wall-light1")
                         .output();
                         Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri-dark $HOME/.config/autowallp/wall-dark1")
                         .output();
            } else if wo == 2 {
                println!("Workspace 2 \n Wall2");
                Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri $HOME/.config/autowallp/wall-light2")
                         .output();
                         Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri-dark $HOME/.config/autowallp/wall-dark2")
                         .output();
            } else if wo == 3 {
                println!("Workspace 3 \n Wall3");
                Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri $HOME/.config/autowallp/wall-light3")
                         .output();
                         Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri-dark $HOME/.config/autowallp/wall-dark3")
                         .output();
            } else if wo == 4 {
                println!("Workspace 4 \n Wall4");
                Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri $HOME/.config/autowallp/wall-light4")
                         .output();
                         Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri-dark $HOME/.config/autowallp/wall-dark4")
                         .output();
            } else if wo == 5 {
                println!("Workspace 0 \n Wall0");
                Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri $HOME/.config/autowallp/wall-light5")
                         .output();
                         Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri-dark $HOME/.config/autowallp/wall-dark5")
                         .output();
            } else if wo == 6 {
                println!("Workspace 1 \n Wall1");
                Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri $HOME/.config/autowallp/wall-light6")
                         .output();
                         Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri-dark $HOME/.config/autowallp/wall-dark6")
                         .output();
            } else if wo == 7 {
                println!("Workspace 2 \n Wall2");
                Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri $HOME/.config/autowallp/wall-light7")
                         .output();
                         Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri-dark $HOME/.config/autowallp/wall-dark7")
                         .output();
            } else if wo == 8 {
                println!("Workspace 3 \n Wall3");
                Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri $HOME/.config/autowallp/wall-light8")
                         .output();
                         Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri-dark $HOME/.config/autowallp/wall-dark8")
                         .output();
            } else if wo == 9 {
                println!("Workspace 4 \n Wall4");
                Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri $HOME/.config/autowallp/wall-light9")
                         .output();
                         Command::new("sh")
                         .arg("-c")
                         .arg("gsettings set org.gnome.desktop.background picture-uri-dark $HOME/.config/autowallp/wall-dark9")
                         .output();
            }
        let mut wn:i64 = wo.clone();
        println!("{}",wn);
        }
    }
}
