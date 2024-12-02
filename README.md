# queryer

A tool to query data from lots of different sources.


## queryer-py

A python binding for queryer.

create a virtual environment and install the dependencies:

```bash
cd queryer-py
python3 -m venv .venv
source .env/bin/activate
pip install maturin ipython
```

build the extension module `queryer_py`:

```bash
maturin develop
```

use `ipython` to test the module:

```bash
ipython

In [1]: import queryer_py;

In [2]: sql = queryer_py.example_sql();

In [3]: print(queryer_py.query(sql, 'csv'));
name,total_cases,new_cases,total_deaths,new_deaths
World,775866783.0,47169.0,7057132.0,815.0
High-income countries,429044049.0,32293.0,2997359.0,786.0
North America,124492666.0,454.0,1671178.0,619.0
United States,103436829.0,,1193165.0,619.0
```
