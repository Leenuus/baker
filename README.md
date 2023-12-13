## Design

1. you should only manage local repo with baker
2. never manually manage local repo, or it will break
3. Setup dirs for setup scripts
4. Give users way to deploy some softwares with corresponding dotfiles, link setup files and config files
5. crazy user experience in chezmoi, i don't commit my tmux, and i add my tmux, but when i call chezmoi remove, all my tmux settings get deleted!!!

## Baker is a dotfiles backuper

file_to_backup --> backup_file_path(bk_path)

bk_path --> local_repo_path
local_repo_path --> bk_path

bk_path --> alias
alias --> local_repo_path
alias --> bk_path

## features

### Encryption
chezmoi supports encrypting files with age and gpg.

Encrypted files are stored in ASCII-armored format in the source directory with the encrypted_ attribute and are automatically decrypted when needed.

Add files to be encrypted with the --encrypt flag, for example:

```shell
chezmoi add --encrypt ~/.ssh/id_rsa
# chezmoi edit will transparently decrypt the file before editing and re-encrypt it afterwards.
```


## TODO

1. Add software setup features
2. Add backup file alias features for quick edit
3. Add verbose mode and dry run mode
4. Add cd, new a shell
5. modify subcommands, give alias, give setup scripts, 
6. a ignore list of useless things in .config dir

### setup

1. user shell
2. user platform
