#!/usr/bin/env zsh
git clone git@github.com:bingecraft-net/antimatter-prune.git $HOME/antimatter
mkdir $HOME/.config
ln -s $HOME/antimatter/.config/nvim $HOME/.config/nvim 
cd $HOME/antimatter

echo 
echo !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
echo 
echo 
echo welcome to the antimatter devcontainer
echo 
echo 
echo !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
echo 

/bin/zsh
