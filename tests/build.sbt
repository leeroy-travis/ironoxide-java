organization := "com.ironcorelabs"
name := "ironoxide-java"
scalaVersion := "2.12.8"

//We're using sbt to test, but this is a pure java library for now so we don't want scala version
//in the paths and we don't want the scala lib in the dependencies.
crossPaths := false
autoScalaLibrary := false

scalacOptions := Seq(
  "-deprecation",
  "-encoding",
  "UTF-8", // yes, this is 2 args
  "-feature",
  "-unchecked",
  "-Xfatal-warnings",
  "-Yno-adapted-args",
  "-Ywarn-numeric-widen",
  "-Ywarn-value-discard",
  "-Xfuture",
  "-language:higherKinds"
)

libraryDependencies ++= Seq(
  "com.pauldijou" %% "jwt-core" % "2.1.0",
  "org.scalatest" %% "scalatest" % "3.0.5",
  "org.scodec" %% "scodec-bits" % "1.1.10",
  "com.github.melrief" %% "pureconfig" %  "0.5.1",
  "com.ironcorelabs" %% "cats-scalatest" % "2.4.0"
).map(_ % "test")
//Include the generated java as part of the source directories
unmanagedSourceDirectories in Compile += baseDirectory.value / ".." / "java"
// HACK: without these lines, the console is basically unusable,
// since all imports are reported as being unused (and then become
// fatal errors).
scalacOptions in (Compile, console) ~= {
  _.filterNot(_.startsWith("-Xlint")).filterNot(_.startsWith("-Ywarn"))
}
scalacOptions in (Test, console) := (scalacOptions in (Compile, console)).value

javaOptions in Test += s"-Djava.library.path=../target/debug/"
fork in Test := true