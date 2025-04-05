package com.example;

import org.apache.spark.SparkConf;
import org.apache.spark.api.java.JavaPairRDD;
import org.apache.spark.api.java.JavaRDD;
import org.apache.spark.api.java.JavaSparkContext;
import org.apache.spark.sql.Dataset;
import org.apache.spark.sql.Row;
import org.apache.spark.sql.RowFactory;
import org.apache.spark.sql.SparkSession;

import org.apache.spark.sql.types.DataTypes;
import org.apache.spark.sql.types.StructField;
import org.apache.spark.sql.types.StructType;
import scala.Tuple2;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class App {
	public static void main(String[] args) {
		Logger.getLogger("org").setLevel(Level.SEVERE);
		
		//----------------------------------Task1 Start------------------------------------
		SparkConf conf = new SparkConf()
			.setAppName("WordIndex")
			.setMaster("local[*]");
		JavaSparkContext sc = new JavaSparkContext(conf);
		List<Tuple2<String, String>> pages = Arrays.asList(
			new Tuple2<>("page1", "word1 word2 word3"),
			new Tuple2<>("page2", "word2 word3 word4"),
			new Tuple2<>("page3", "word3 word4 word5")
		);
		JavaRDD<Tuple2<String, String>> pagesRDD = sc.parallelize(pages);
		
		JavaPairRDD<String, String> wordsToPagesRDD = pagesRDD.flatMapToPair(
			page -> Arrays.stream(page._2().split(" "))
			.map(word -> new Tuple2<>(word, page._1()))
			.iterator()
		);
 
		JavaPairRDD<String, Iterable<String>> indexRDD = wordsToPagesRDD.groupByKey();
 
		indexRDD.foreach(pair -> System.out.println(pair._1() + ": " + pair._2()));

		indexRDD.saveAsTextFile("task1");
		//----------------------------------Task1 End--------------------------------------
 
		//----------------------------------Task3 Start------------------------------------
		JavaRDD<String> pagesWithWord2RDD = wordsToPagesRDD.filter(
			pair -> pair._1().equals("word2"))
			.map(pair -> pair._2());
		pagesWithWord2RDD.foreach(page -> System.out.println(page));
		pagesWithWord2RDD.saveAsTextFile("task3");
		
		//----------------------------------Task3 End--------------------------------------
 
		//----------------------------------Task2 Start------------------------------------
		SparkSession spark = SparkSession.builder()
			.appName("Java Spark SQL Example")
			.config("spark.master", "local")
		.getOrCreate();
 
		String[] metadata = {"metadata1", "metadata2", "metadata3"};
		String[] links = {"link1", "link2", "link3"};
		Row[] rows = new Row[metadata.length];
		for (int i = 0; i < metadata.length; i++) {
			rows[i] = RowFactory.create(metadata[i], links[i]);
		}
		List<StructField> fields = new ArrayList<>();
		fields.add(DataTypes.createStructField("metadata",
			DataTypes.StringType, true));
		fields.add(DataTypes.createStructField("link", DataTypes.StringType,
			true));
		StructType schema = DataTypes.createStructType(fields);
		Dataset<Row> dataset = spark.createDataFrame(Arrays.asList(rows),
			schema);
		
		dataset.createOrReplaceTempView("metadata_links");

		Dataset<Row> result = spark.sql("SELECT * FROM metadata_links");
		result.show();

		result.write().csv("task2");
		//----------------------------------Task2 End-------------------------------------
 
		//----------------------------------Task4 Start------------------------------------

		Dataset<Row> results = spark.sql("SELECT * FROM metadata_links WHERE metadata = 'metadata2'");

		results.write().mode("overwrite").csv("task4");
		//----------------------------------Task4 End--------------------------------------
 

		sc.close();
	}
}