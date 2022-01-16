box.cfg {
    listen = 3301,
    wal_mode = 'none',
}

box.schema.func.create('csum', { language = 'C', if_not_exists = true })

function sum(a, b)
    return { a + b }
end

box.schema.user.grant('guest', 'super', nil, nil, { if_not_exists = true })
