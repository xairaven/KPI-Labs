package com.example;
import java.io.IOException;
import org.apache.hadoop.conf.Configuration;
import org.apache.hadoop.fs.Path;
import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.NullWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Job;
import org.apache.hadoop.mapreduce.Mapper;
import org.apache.hadoop.mapreduce.Reducer;
import org.apache.hadoop.mapreduce.lib.input.TextInputFormat;
import org.apache.hadoop.mapreduce.lib.output.TextOutputFormat;
public class App {
	
  // Mapper Class
  public static class CountMapper extends Mapper<Object, Text, Text, IntWritable> {
      private final static IntWritable one = new IntWritable(1);
      private final static IntWritable zero = new IntWritable(0);
      public void map(Object key, Text value, Context context) throws
      IOException,
      InterruptedException {
        int number = Integer.parseInt(value.toString());
        if (number > 60) {
          context.write(new Text("greater than 60"), one);
        } else {
          context.write(new Text("less than or equal to 60"), one);
        }
      }
    }
	
  // Reducer Class
  public static class CountReducer extends Reducer<Text, IntWritable, Text, IntWritable> {
      public void reduce(Text key, Iterable<IntWritable> values, Context context) throws IOException,
      InterruptedException {
        int count = 0;
        for (IntWritable value: values) {
          count += value.get();
        }
        context.write(key, new IntWritable(count));
      }
    }
  // Main Method
  public static void main(String[] args) throws Exception {
    Configuration conf = new Configuration();
    conf.set("fs.defaultFS", "hdfs://namenode:9000");
    Job job = Job.getInstance(conf, "App");
    job.setJarByClass(App.class);
    job.setMapperClass(CountMapper.class);
    job.setReducerClass(CountReducer.class);
    job.setOutputKeyClass(Text.class);
    job.setOutputValueClass(IntWritable.class);
    job.setInputFormatClass(TextInputFormat.class);
    job.setOutputFormatClass(TextOutputFormat.class);
    TextInputFormat.addInputPath(job, new Path(args[0]));
    TextOutputFormat.setOutputPath(job, new Path(args[1]));
    System.exit(job.waitForCompletion(true) ? 0 : 1);
  }
}