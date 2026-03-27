package ua.kpi.lab6

import android.content.ContentValues
import android.content.Context
import android.database.sqlite.SQLiteDatabase
import android.database.sqlite.SQLiteOpenHelper

// Helper class to manage the SQLite database for reminders
class DatabaseHelper(context: Context) :
    SQLiteOpenHelper(context, DATABASE_NAME, null, DATABASE_VERSION) {

    companion object {
        private const val DATABASE_NAME = "reminders.db"
        private const val DATABASE_VERSION = 1

        const val TABLE_NAME = "reminders_table"
        const val COLUMN_ID = "id"
        const val COLUMN_THEME = "theme"
        const val COLUMN_MESSAGE = "message"
        const val COLUMN_DATE = "remind_date"
    }

    override fun onCreate(db: SQLiteDatabase) {
        // Create table query for reminders
        val createTableQuery = ("CREATE TABLE $TABLE_NAME ("
                + "$COLUMN_ID INTEGER PRIMARY KEY AUTOINCREMENT, "
                + "$COLUMN_THEME TEXT, "
                + "$COLUMN_MESSAGE TEXT, "
                + "$COLUMN_DATE TEXT)")
        db.execSQL(createTableQuery)
    }

    override fun onUpgrade(db: SQLiteDatabase, oldVersion: Int, newVersion: Int) {
        db.execSQL("DROP TABLE IF EXISTS $TABLE_NAME")
        onCreate(db)
    }

    // Insert a new reminder into the database
    fun insertReminder(theme: String, message: String, date: String): Long {
        val db = this.writableDatabase
        val values = ContentValues()
        values.put(COLUMN_THEME, theme)
        values.put(COLUMN_MESSAGE, message)
        values.put(COLUMN_DATE, date)
        val id = db.insert(TABLE_NAME, null, values)
        db.close()
        return id
    }

    // Retrieve all reminders as a formatted string
    fun getAllReminders(): String {
        val db = this.readableDatabase
        val cursor = db.rawQuery("SELECT * FROM $TABLE_NAME ORDER BY $COLUMN_ID DESC", null)
        val stringBuilder = StringBuilder()

        if (cursor.moveToFirst()) {
            do {
                val id = cursor.getInt(cursor.getColumnIndexOrThrow(COLUMN_ID))
                val theme = cursor.getString(cursor.getColumnIndexOrThrow(COLUMN_THEME))
                val message = cursor.getString(cursor.getColumnIndexOrThrow(COLUMN_MESSAGE))
                val date = cursor.getString(cursor.getColumnIndexOrThrow(COLUMN_DATE))
                stringBuilder.append("ID: $id | Тема: $theme\nЧас: $date\nТекст: $message\n\n")
            } while (cursor.moveToNext())
        } else {
            stringBuilder.append("Немає збережених нагадувань.")
        }
        cursor.close()
        db.close()
        return stringBuilder.toString()
    }

    // Delete a specific reminder by its ID
    fun deleteReminder(id: Int): Boolean {
        val db = this.writableDatabase
        val result = db.delete(TABLE_NAME, "$COLUMN_ID = ?", arrayOf(id.toString()))
        db.close()
        return result > 0
    }
}