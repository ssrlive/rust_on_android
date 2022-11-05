package com.example.myrustapp;

public class RustBindings {
    static {
        System.loadLibrary("myrustlib");
    }

    public static native String greeting(final String pattern);
}