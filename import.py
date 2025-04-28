import json
import sqlite3


conn = sqlite3.connect('database.db')
cursor = conn.cursor()
cursor.execute('PRAGMA foreign_keys=on')

path_to_prompts = '../frottage/prompts.json'


with open(path_to_prompts, 'r', encoding='utf-8') as f:
    for line in f:
        data = json.loads(line)
        print(data)
        cursor.execute('''
           insert into prompt (prompt)
           values (:prompt)
        ''', data)


path_to_targets = '../frottage/targets.json'


with open(path_to_targets, 'r', encoding='utf-8') as f:
    for line in f:
        data = json.loads(line)
        cursor.execute('''
           insert into target (target_name, aspect_ratio)
           values (:name, :aspectRatio)
        ''', data)


with open(path_to_prompts, 'r', encoding='utf-8') as f:
    for line in f:
        data = json.loads(line)
        print(data["prompt"])
        cursor.execute("select prompt_id from prompt where prompt=:prompt", data)
        id = cursor.fetchone()[0]
        for target in data["targets"]:
            print(id)
            print(target)
            cursor.execute('''
                insert into prompt_target (prompt_id, target_name)
                values (:id, :target)
            ''', {'id':id, 'target':target})

conn.commit()
conn.close()
