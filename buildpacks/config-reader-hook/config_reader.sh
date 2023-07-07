#!/bin/bash

# Require: pack, jq, tr

repo_path="." # A hook is always run in the root directory of the repo

config=`cat "$repo_path/.paastech/config.json"`
subrepos=`jq ".subrepos" <<< $config`
appName=`echo $config | jq ".name" | tr -d '"'`

repos_count=`jq length <<< $subrepos`

for (( i=0; i < $repos_count; i++ ))
do
    subrepo=`jq ".subrepos[$i]" <<< $config`
    name=`echo $subrepo | jq ".name" | tr -d '"'`
    path=`echo $subrepo | jq ".path" | tr -d '"'`

    path_format=`tr -d '"' <<< $path`
    absolute_path="$repo_path/$path_format"
    filename=`basename $absolute_path`
    
    imagename=`tr '[:upper:]' '[:lower:]' <<< $appName-$name`
    pack build "$imagename" -p "$absolute_path" --builder paketobuildpacks/builder:base
done