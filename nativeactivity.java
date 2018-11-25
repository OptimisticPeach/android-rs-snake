package rust.{package_name};

import java.lang.UnsupportedOperationException;
import android.os.Build;
import android.util.Log;
import android.app.NativeActivity;
import android.os.Bundle;
import android.view.WindowManager;
import android.view.View;

public class MainActivity extends NativeActivity {{
    public void onWindowFocusChanged(boolean hasFocus){{
        super.onWindowFocusChanged(hasFocus);
        getWindow().addFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN);
        runOnUiThread(new Runnable() {{

            @Override
            public void run() {{
                View decorView = getWindow().getDecorView();
                // Hide both the navigation bar and the status bar.
                // SYSTEM_UI_FLAG_FULLSCREEN is only available on Android 4.1 and higher, but as
                // a general rule, you should design your app to hide the status bar whenever you
                // hide the navigation bar.
                int uiOptions = View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
                              | View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
                              | View.SYSTEM_UI_FLAG_FULLSCREEN;
                decorView.setSystemUiVisibility(uiOptions);
            }}
        }});
    }}
    static {{

        String[] supported_abis;

        try {{
            supported_abis = (String[]) Build.class.getField("SUPPORTED_ABIS").get(null);
        }} catch (Exception e) {{
            // Assume that this is an older phone; use backwards-compatible targets.
            supported_abis = new String[]{{Build.CPU_ABI, Build.CPU_ABI2}};
        }}

        boolean matched_an_abi = false;

        for (String abi : supported_abis) {{
{libs}
        }}

        if (!matched_an_abi) {{
            throw new UnsupportedOperationException("Could not find a native abi target to load");
        }}

    }}
}}