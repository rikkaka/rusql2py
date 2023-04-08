# rusql2py
## Usage
Ensure you are in an env first.
```shell
git clone https://github.com/rikkaka/rusql2py
cd rusql2py
pip install maturin
maturin develop
```
Then rusql2py is installed as a py module.
```python
import rusql2py

conn = rusql2py.connect("test.db") 
conn.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)", [])
conn.execute("INSERT INTO test (name) VALUES (?)", ["test"])
```
