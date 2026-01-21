# Keep the UniFFI generated classes and their init functions
-keep class rs.padauk.** { *; }
-keepclassmembers class rs.padauk.** {
    public static void padaukInit();
}
-keepclasseswithmembernames class * {
    native <methods>;
}