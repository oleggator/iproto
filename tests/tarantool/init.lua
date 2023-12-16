require('fio').mkdir('data')
box.cfg {
    listen = 3301,
    wal_mode = 'none',
    wal_dir = 'data',
    snap_dir = 'data',
}

box.schema.func.create('procedures.sum', { language = 'C', if_not_exists = true })
box.schema.func.create('procedures.echo', { language = 'C', if_not_exists = true })
box.schema.func.create('sum', { language = 'Lua', if_not_exists = true })
box.schema.func.create('echo', { language = 'Lua', if_not_exists = true })

function sum(a, b)
    return { a + b }
end

function echo(...)
    return ...
end

-- box.func['procedures.echo']:call{1,2}
-- box.func['procedures.sum']:call{1,2}

box.schema.user.grant('guest', 'super', nil, nil, { if_not_exists = true })
