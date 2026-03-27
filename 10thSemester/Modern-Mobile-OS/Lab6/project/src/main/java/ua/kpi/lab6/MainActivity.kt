package ua.kpi.lab6

import android.app.AlarmManager
import android.app.DatePickerDialog
import android.app.PendingIntent
import android.app.TimePickerDialog
import android.content.Context
import android.content.Intent
import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import java.text.SimpleDateFormat
import java.util.Calendar
import java.util.Locale

class MainActivity : AppCompatActivity() {

    private lateinit var dbHelper: DatabaseHelper
    private lateinit var tvAllReminders: TextView
    private lateinit var tvDateTime: TextView

    private val calendar = Calendar.getInstance()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        dbHelper = DatabaseHelper(this)

        val etTheme = findViewById<EditText>(R.id.etTheme)
        val etMessage = findViewById<EditText>(R.id.etMessage)
        val etDeleteId = findViewById<EditText>(R.id.etDeleteId)

        tvAllReminders = findViewById(R.id.tvAllReminders)
        tvDateTime = findViewById(R.id.tvDateTime)

        val btnDate = findViewById<Button>(R.id.btnDate)
        val btnTime = findViewById<Button>(R.id.btnTime)
        val btnSave = findViewById<Button>(R.id.btnSave)
        val btnDelete = findViewById<Button>(R.id.btnDelete)

        updateRemindersList()

        btnDate.setOnClickListener {
            val year = calendar.get(Calendar.YEAR)
            val month = calendar.get(Calendar.MONTH)
            val day = calendar.get(Calendar.DAY_OF_MONTH)

            // Show date picker dialog
            DatePickerDialog(this, { _, selectedYear, selectedMonth, selectedDay ->
                calendar.set(Calendar.YEAR, selectedYear)
                calendar.set(Calendar.MONTH, selectedMonth)
                calendar.set(Calendar.DAY_OF_MONTH, selectedDay)
                updateDateTimeText()
            }, year, month, day).show()
        }

        btnTime.setOnClickListener {
            val hour = calendar.get(Calendar.HOUR_OF_DAY)
            val minute = calendar.get(Calendar.MINUTE)

            // Show time picker dialog
            TimePickerDialog(this, { _, selectedHour, selectedMinute ->
                calendar.set(Calendar.HOUR_OF_DAY, selectedHour)
                calendar.set(Calendar.MINUTE, selectedMinute)
                updateDateTimeText()
            }, hour, minute, true).show()
        }

        btnSave.setOnClickListener {
            val theme = etTheme.text.toString()
            val message = etMessage.text.toString()
            val timeString = tvDateTime.text.toString()

            if (theme.isNotEmpty() && message.isNotEmpty() && timeString != "Час не обрано") {
                // Save to database
                dbHelper.insertReminder(theme, message, timeString)

                // Schedule system alarm
                scheduleNotification(theme, message, calendar.timeInMillis)

                Toast.makeText(this, "Нагадування збережено", Toast.LENGTH_SHORT).show()
                updateRemindersList()
            } else {
                Toast.makeText(this, "Заповніть всі поля та оберіть час", Toast.LENGTH_SHORT).show()
            }
        }

        btnDelete.setOnClickListener {
            val idStr = etDeleteId.text.toString()
            if (idStr.isNotEmpty()) {
                val success = dbHelper.deleteReminder(idStr.toInt())
                if (success) {
                    Toast.makeText(this, "Видалено успішно", Toast.LENGTH_SHORT).show()
                    updateRemindersList()
                } else {
                    Toast.makeText(this, "Помилка видалення", Toast.LENGTH_SHORT).show()
                }
            }
        }
    }

    // Refresh the text view displaying all database entries
    private fun updateRemindersList() {
        tvAllReminders.text = dbHelper.getAllReminders()
    }

    // Update the UI text indicating chosen time
    private fun updateDateTimeText() {
        val format = SimpleDateFormat("dd.MM.yyyy HH:mm", Locale.getDefault())
        tvDateTime.text = format.format(calendar.time)
    }

    // Configure AlarmManager to trigger the receiver at specific time
    private fun scheduleNotification(theme: String, message: String, timeInMillis: Long) {
        val intent = Intent(this, ReminderReceiver::class.java).apply {
            putExtra("theme", theme)
            putExtra("message", message)
        }

        val pendingIntent = PendingIntent.getBroadcast(
            this,
            timeInMillis.toInt(),
            intent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        val alarmManager = getSystemService(Context.ALARM_SERVICE) as AlarmManager

        // Use exact alarm for precise notification timing
        try {
            alarmManager.setExact(AlarmManager.RTC_WAKEUP, timeInMillis, pendingIntent)
        } catch (e: SecurityException) {
            Toast.makeText(this, "Немає дозволу на точні будильники", Toast.LENGTH_SHORT).show()
        }
    }
}