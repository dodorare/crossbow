// package java.com.permissions;

// import androidx.appcompat.app.AppCompatActivity;
// import android.graphics.Color;
// import android.os.Bundle;
// import android.view.View;
// import android.widget.Button;
// import android.widget.EditText;
// import android.widget.TextView;

// public class MainActivity extends AppCompatActivity {
// 	private Button button;
// 	private EditText input;
// 	private TextView resultBox;

// 	private int colourRed;
// 	private int colourGreen;

// 	@Override
// 	protected void onCreate(Bundle savedInstanceState) {
// 		super.onCreate(savedInstanceState);
// 		setContentView(R.layout.activity_main);

// 		button = (Button)findViewById(R.id.button);
// 		input = (EditText)findViewById(R.id.exprInput);
// 		resultBox = (TextView)findViewById(R.id.exprResult);

// 		colourRed = Color.parseColor("#AA0000");
// 		colourGreen = Color.parseColor("#007F00");

// 		button.setOnClickListener(new View.OnClickListener() {
// 			public void onClick(View v) {
// 				String expr = input.getText().toString();
// 				Result result = RpnCalculator.rpn(expr);
// 				if(result.isOk()) {
// 					resultBox.setTextColor(colourGreen);
// 					resultBox.setText(result.getValue());
// 				} else {
// 					resultBox.setTextColor(colourRed);
// 					resultBox.setText(result.getError());
// 				}
// 			}
// 		});
// 	}
// }
