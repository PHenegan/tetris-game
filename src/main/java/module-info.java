module com.example.tetris {
  requires javafx.controls;
  requires javafx.fxml;


  opens com.phenegan.tetris to javafx.fxml;
  exports com.phenegan.tetris;
}