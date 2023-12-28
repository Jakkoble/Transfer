# Transfer

A simple Rust CLI tool for moving and copying files or directories from your Windows Subsystem for Linux (WSL2) to your Windows Machine. You might ask yourself if you couldn't just pin your home directory to your Windows directory to Drag and Drop it to your Desktop. 
<br /><br />
**Simply said: After you get used to this Tool, the repetitive task of copy and paste doesn't exist anymore.**<br /><br />
_Do I even have to mention the annoying Security Warnings when dealing with manual copy and pasting?_
<br /> <br />

## Setup

1. Open up the WSL Terminal
2. Install `cURL` using: `sudo apt install curl`
3. Download the tool: `curl https://github.com/jakkoble/transfer/releases/latest/download/transfer`
4. Move it to the User Binaries Directory with `sudo mv transfer /usr/local/bin`
5. Type `transfer` to execute the CLI

<br />

## Usage
By default, you just have to specify the file/directory and it gets moved to your Windows Desktop. If you want a different behavior, you can specify options. (See Screenshot)
#### Display Help Message
```
$ transfer -h
```
![Terminal](https://i.imgur.com/MBQT2PN.png)
<br />
#### Move file to Windows Desktop
```
$ transfer my_script.sh
```
#### Copy file to Windows Desktop
```
$ transfer -c my_script.sh
```
#### Copy file to Windows Desktop with Custom Name
```
$ transfer -c my_script.sh perfect_script
```
#### Force copy file to Windows Desktop
```
$ transfer -c -f my_script.sh
```
<br /><br />
If you need further help or you find a bug, feel free to **[open a new Issue](https://github.com/Jakkoble/Transfer/issues/new)**. 
