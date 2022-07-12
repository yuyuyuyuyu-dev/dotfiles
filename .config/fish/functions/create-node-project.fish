function create-node-project
    mkdir {$argv[1]}
    cd {$argv[1]}

    npm init -y
    npm install -D typescript @types/node ts-node
    npx tsc --init
    curl -o 'tsconfig.json' 'https://raw.githubusercontent.com/yu-ko-ba/tsconfig/main/tsconfig.json'
    volta pin node
end
