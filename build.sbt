inThisBuild {
  scalaVersion := "3.4.1"
}

lazy val scion = (project in file("src/scala"))
  .enablePlugins(ScalaNativePlugin)
  .settings(
    libraryDependencies ++= Seq(
      "dev.zio" %% "zio" % "2.1.5",
      "dev.zio" %% "zio-nio" % "2.0.2",
      "pt.kcry" %% "blake3" % "3.1.2"
    )
  )

//lazy val engine = (project in file("src/scala/engine"))
