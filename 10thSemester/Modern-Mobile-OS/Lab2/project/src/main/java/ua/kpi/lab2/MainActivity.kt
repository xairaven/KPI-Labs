package ua.kpi.lab2

import android.content.Intent
import android.os.Bundle
import android.widget.Button
import androidx.activity.enableEdgeToEdge
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContentView(R.layout.activity_main)

        // Button 1
        findViewById<Button>(R.id.btnToLinear).setOnClickListener {
            startActivity(Intent(this, SecondActivity::class.java))
        }

        // Button 2
        findViewById<Button>(R.id.btnToRelative).setOnClickListener {
            startActivity(Intent(this, ThirdActivity::class.java))
        }

        // Button 3
        findViewById<Button>(R.id.btnToStyle).setOnClickListener {
            startActivity(Intent(this, FourthActivity::class.java))
        }

        // Button 4
        findViewById<Button>(R.id.btnExit).setOnClickListener {
            finishAffinity()
        }
    }
}