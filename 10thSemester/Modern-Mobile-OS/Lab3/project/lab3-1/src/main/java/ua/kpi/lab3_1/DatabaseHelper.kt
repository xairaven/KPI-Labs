package ua.kpi.lab3_1

import android.content.ContentValues
import android.content.Context
import android.database.sqlite.SQLiteDatabase
import android.database.sqlite.SQLiteOpenHelper

// Helper class for managing SQLite database
class DatabaseHelper(context: Context) :
    SQLiteOpenHelper(context, DATABASE_NAME, null, DATABASE_VERSION) {
    companion object {
        private const val DATABASE_NAME = "university.db"
        private const val DATABASE_VERSION = 1

        // Table and columns names
        private const val TABLE_NAME = "classmates"
        private const val COLUMN_ID = "id"
        private const val COLUMN_NAME = "name"
        private const val COLUMN_TIME = "time_added"
    }

    override fun onCreate(db: SQLiteDatabase) {
        // Create table query
        val createTableQuery = ("CREATE TABLE $TABLE_NAME ("
                + "$COLUMN_ID INTEGER PRIMARY KEY AUTOINCREMENT, "
                + "$COLUMN_NAME TEXT, "
                + "$COLUMN_TIME TEXT)")
        db.execSQL(createTableQuery)
    }

    override fun onUpgrade(db: SQLiteDatabase, oldVersion: Int, newVersion: Int) {
        // Drop older table if exists and create fresh
        db.execSQL("DROP TABLE IF EXISTS $TABLE_NAME")
        onCreate(db)
    }

    // Delete all records from the table
    fun clearDatabase() {
        val db = this.writableDatabase
        db.execSQL("DELETE FROM $TABLE_NAME")
        db.close()
    }

    // Insert a new student record
    fun addStudent(name: String, time: String) {
        val db = this.writableDatabase
        val values = ContentValues()
        values.put(COLUMN_NAME, name)
        values.put(COLUMN_TIME, time)
        db.insert(TABLE_NAME, null, values)
        db.close()
    }

    // Update the last added record's name
    fun updateLastStudent(newName: String) {
        val db = this.writableDatabase
        // Update where ID is the maximum ID in the table
        val updateQuery =
            "UPDATE $TABLE_NAME SET $COLUMN_NAME = '$newName' WHERE $COLUMN_ID = (SELECT MAX($COLUMN_ID) FROM $TABLE_NAME)"
        db.execSQL(updateQuery)
        db.close()
    }

    // Retrieve all students as a formatted string
    fun getAllStudents(): String {
        val db = this.readableDatabase
        val cursor = db.rawQuery("SELECT * FROM $TABLE_NAME", null)
        val stringBuilder = StringBuilder()

        if (cursor.moveToFirst()) {
            do {
                val id = cursor.getInt(cursor.getColumnIndexOrThrow(COLUMN_ID))
                val name = cursor.getString(cursor.getColumnIndexOrThrow(COLUMN_NAME))
                val time = cursor.getString(cursor.getColumnIndexOrThrow(COLUMN_TIME))
                stringBuilder.append("ID: $id\nName: $name\nTime: $time\n\n")
            } while (cursor.moveToNext())
        }
        cursor.close()
        db.close()

        return stringBuilder.toString()
    }
}