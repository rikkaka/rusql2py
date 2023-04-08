import rusql2py

conn = rusql2py.connect("test.db") 
conn.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)", [])
conn.execute("INSERT INTO test (name) VALUES (?)", ["test"])