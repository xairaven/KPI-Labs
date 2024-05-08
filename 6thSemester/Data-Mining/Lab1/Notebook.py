import sqlite3

db_filename = 'data/cereals-1.db'

conn = sqlite3.connect(db_filename)
c = conn.cursor()

c.execute('''
    SELECT DISTINCT Manufacturer FROM cereals
''')

for manufacturer in c.fetchall():
    c.execute('''SELECT avg(Sugars)
        FROM cereals WHERE
        Manufacturer='{0}'
        '''.format(manufacturer[0]))

    print("{0}: {1:.2f}"
          .format(manufacturer[0], c.fetchall()[0][0]))