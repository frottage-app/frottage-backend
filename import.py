import json
import sqlite3


conn = sqlite3.connect('database.db')
cursor = conn.cursor()
cursor.execute('PRAGMA foreign_keys=on')

path_to_prompts = '../frottage/prompts.json'

cursor.execute('''
CREATE TABLE prompts
(id integer primary key, prompt text unique not null);
''')

with open(path_to_prompts, 'r', encoding='utf-8') as f:
    for line in f:
        data = json.loads(line)
        print(data)
        cursor.execute('''
           insert into prompts (prompt)
           values (:prompt)
        ''', data)


path_to_targets = '../frottage/targets.json'

cursor.execute('''
CREATE TABLE targets
(name text primary key, aspectRatio text not null,
CHECK(
    aspectRatio like '%:%' and
    cast(cast(substr(aspectRatio, 1, instr(aspectRatio, ':')-1) as integer) as text) = substr(aspectRatio, 1, instr(aspectRatio, ':')-1) and
    cast(cast(substr(aspectRatio, instr(aspectRatio, ':')+1) as integer) as text) = substr(aspectRatio, instr(aspectRatio, ':')+1)
)
);
''')


with open(path_to_targets, 'r', encoding='utf-8') as f:
    for line in f:
        data = json.loads(line)
        cursor.execute('''
           insert into targets (name, aspectRatio)
           values (:name, :aspectRatio)
        ''', data)


cursor.execute('''
CREATE TABLE promptTargets
(promptId integer not null , targetName text not null,
foreign key(promptId) references prompts(id) on delete cascade on update cascade,
foreign key(targetName) references targets(name) on delete cascade on update cascade)
''')

with open(path_to_prompts, 'r', encoding='utf-8') as f:
    for line in f:
        data = json.loads(line)
        print(data["prompt"])
        cursor.execute("select id from prompts where prompt=:prompt", data)
        id = cursor.fetchone()[0]
        for target in data["targets"]:
            print(id)
            print(target)
            cursor.execute('''
                insert into promptTargets (promptId, targetName)
                values (:id, :target)
            ''', {'id':id, 'target':target})

conn.commit()
conn.close()
