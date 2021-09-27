package hk.jennyemily.wumpus;

import androidx.annotation.NonNull;
import androidx.appcompat.app.AppCompatActivity;

import android.app.Activity;
import android.os.Bundle;
import android.util.Base64;
import android.util.Log;
import android.webkit.ConsoleMessage;
import android.webkit.JavascriptInterface;
import android.webkit.JsResult;
import android.webkit.WebSettings;
import android.webkit.WebView;
import android.webkit.WebViewClient;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

import java.io.File;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.io.OutputStream;
import java.util.ArrayList;
import java.util.HashMap;
import wumpus.*;

public class MainActivity extends AppCompatActivity {

    private final static String TAG = "Wumpus";

    static {
        try {
            System.loadLibrary("wumpus");
        } catch (Exception e) {
            Log.e(TAG, "exception in load library" + e);
        }
    }

    private SWIGTYPE_p_UserHandle_Engine td;
    private WebView mWebView;
    private String appStatus = "initial";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        try {
            Log.d(TAG, "on create...");
            super.onCreate(savedInstanceState);

            setContentView(R.layout.activity_main);

            mWebView = findViewById(R.id.webview);
            mWebView.setVerticalScrollBarEnabled(true);
            WebSettings webSettings = mWebView.getSettings();
            webSettings.setJavaScriptEnabled(true);
            mWebView.addJavascriptInterface(new WebAppInterface(this), "wumpus");
            mWebView.setWebViewClient(new WebViewClient() {
                boolean onConsoleMessage(ConsoleMessage consoleMessage) {
                    Log.d(TAG, "console message " + consoleMessage);
                    return false;
                }

                boolean onJsAlert(WebView view, String url, String message, JsResult result) {
                    result.confirm();
                    Log.d(TAG, "JS alert suppressed " + message);
                    return true;
                }

                public void onPageFinished(WebView view,
                                           String url) {
                    Log.d(TAG, "web view all loaded");
                    super.onPageFinished(view, url);
                }
            });
            //   mWebView.setWebChromeClient(new WebChromeClient() { });
            new Thread() {
                @Override
                public void run() {
                    Log.d(TAG, "running on worker thread");
                    Log.d(TAG, "creating config");
                    String optionsString = makeConfig();

                    td = wumpus.new_engine(optionsString);
                    Log.d(TAG, "new engine created.");

                    // if (savedInstanceState != null) mWebView.restoreState(savedInstanceState);
                    Log.d(TAG, "app status is " + appStatus);
                    if (appStatus.equals("initial")) {
                        String ih = wumpus.initial_html(td);
                        Log.d(TAG, String.format("have initial html, displaying, initial html len : %d", ih.length()));
                        if (ih.length() > 100)
                            Log.d(TAG, String.format("initial html: %s...", ih.substring(0, 300)));
                        //  Log.d(TAG, ih); // lots of output (all of initial html)
                        //   mWebView.loadData(ih, null, null);
                        String encodedHtml = Base64.encodeToString(ih.getBytes(), Base64.NO_PADDING);
                        mWebView.loadData(encodedHtml, "text/html", "base64");
                        Log.d(TAG, "web view loaded, content height " + mWebView.getContentHeight());
                        appStatus = "started";

                    }
                    //   final Bundle sis = savedInstanceState;
                    // restoreInstanceState(sis);
                    wumpus.handle_event(td, "\"Create\"");
                    Log.d(TAG, "worker thread finished");
                }
            }.run();
            Log.d(TAG, "on create done.");
        } catch (
                Exception e) {
            Log.e(TAG, "exception in on create");
        }
    }

    String makeConfig() {
        JSONObject config_json = new JSONObject();
        try {
        } catch (
                Exception e) {
            Log.e(TAG, "error in config " + e);
        }

        String optionsString = config_json.toString();
        Log.d(TAG, "options are: " + optionsString);
        return optionsString;

    }
    @Override
    public void onSaveInstanceState(@NonNull Bundle outState) {
        wumpus.handle_event(td, "\"SaveInstanceState\"");
      //  try {
            String instance_state = wumpus.last_response_json(td);
            Log.d(TAG, "instance state " + instance_state);
           /* JSONObject response = new JSONObject(instance_state);
            JSONArray tags = response.getJSONArray("tag_vec");
            ArrayList<String> keys = new ArrayList();
            for (int ix = 0; ix < tags.length(); ix++) {
                JSONArray item = tags.getJSONArray(ix);
                String key = item.getString(0);
                if (key == "keys") Log.e(TAG, "cannot user 'keys' as a key");
                String value = item.getString(1);
                Log.d(TAG, "state " + key + " = " + value);
                outState.putString(key, value);
                keys.add(key);
            }
            outState.putStringArrayList("keys", keys);
            Log.d(TAG, "have instance state");*/
      /*  } catch (JSONException e) {
            Log.e(TAG, "bad JSON for instance state");
            e.printStackTrace();
        }*/
        super.onSaveInstanceState(outState);
    }

    @Override
    protected void onRestoreInstanceState(@NonNull Bundle savedInstanceState) {
        restoreInstanceState(savedInstanceState);
        super.onRestoreInstanceState(savedInstanceState);
    }

    void restoreInstanceState(Bundle savedInstanceState) {
        Log.d(TAG, "maybe restoring instance state");
        if (savedInstanceState == null) return;
        Log.d(TAG, "restoring instance state");
        String[] keys = (String[]) savedInstanceState.getStringArrayList("keys").toArray();
        HashMap restore = new HashMap();
        for (int ix = 0; ix < keys.length; ix++) {
            String key = keys[ix];
            String value = savedInstanceState.getString(key);
            restore.put(key, value);
            Log.d(TAG, "restoring " + key + " = " + value);
        }
        wumpus.handle_event(td, "{\"RestoreInstanceState\": " + new JSONObject(restore) + "}");
    }
    class WebAppInterface {
        WebAppInterface(Activity a) {
            Log.d(TAG, "creating web app interface");
        }

        @JavascriptInterface
        public void execute(String body) {
            Log.d(TAG,"invoking [from Java]");
            final String body1 = body;
            new Thread() {
                @Override
                public void run() {
                    Log.d(TAG, "execute running on worker thread [from Java]...");
                    wumpus.execute(td, body1);
                    Log.d(TAG, "execute done [from Java].");
                    if (wumpus.is_shutdown_required(td)) {
                        Log.d(TAG, "shutdown required, deleting engine [from Java]...");
                        wumpus.delete_engine(td);
                        Log.d(TAG, "engine deleted [from Java]");
                        finish();
                    }
//                    String response = wumpus.last_response_json(td);
//                    Log.d(TAG, "received response [from Java] " + response);

                    Log.d(TAG, "worker thread finished [from Java]");
                }
            }.run();
        }

        @JavascriptInterface
        public String last_string() {
            return wumpus.last_string(td);
        }

        @JavascriptInterface
        public String last_response_json() {
            return wumpus.last_response_json(td);
        }


    }

}
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
