package ua.kpi.lab3_1

import android.content.Intent
import android.os.Bundle
import android.widget.Button
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale

class MainActivity : AppCompatActivity() {

    private lateinit var dbHelper: DatabaseHelper

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        dbHelper = DatabaseHelper(this)

        // Clear database and insert 5 initial records on startup
        dbHelper.clearDatabase()
        insertInitialData()

        val btnShowData = findViewById<Button>(R.id.btnShowData)
        val btnAddRecord = findViewById<Button>(R.id.btnAddRecord)
        val btnUpdateRecord = findViewById<Button>(R.id.btnUpdateRecord)

        // Button 1: Open new activity to display data
        btnShowData.setOnClickListener {
            val intent = Intent(this, DisplayActivity::class.java)
            startActivity(intent)
        }

        // Button 2: Add one more record
        btnAddRecord.setOnClickListener {
            val currentTime = getCurrentTime()
            dbHelper.addStudent("Новий Студент", currentTime)
            Toast.makeText(this, "Record added", Toast.LENGTH_SHORT).show()
        }

        // Button 3: Update the last record
        btnUpdateRecord.setOnClickListener {
            dbHelper.updateLastStudent("Петренко Петро Петрович")
            Toast.makeText(this, "Last record updated", Toast.LENGTH_SHORT).show()
        }
    }

    // Helper method to insert 5 students
    private fun insertInitialData() {
        val students =
            listOf("Іванов Іван", "Шевченко Тарас", "Косач Лариса", "Франко Іван", "Стус Василь")
        for (student in students) {
            dbHelper.addStudent(student, getCurrentTime())
        }
    }

    // Helper method to get formatted current time
    private fun getCurrentTime(): String {
        val sdf = SimpleDateFormat("yyyy-MM-dd HH:mm:ss", Locale.getDefault())
        return sdf.format(Date())
    }
}