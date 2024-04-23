using UnityEngine;
using UnityEngine.SceneManagement;

public class ConfigPanel : MonoBehaviour
{
    // Método para iniciar el juego o volver al inicio.
    public void StartGame()
    {
        Debug.Log("Inicio del juego");
        SceneManager.LoadScene("Login"); 
    }

    // Método para silenciar o activar el sonido del juego.
    public void ToggleMute()
    {
        bool isMuted = AudioListener.volume > 0;
        AudioListener.volume = isMuted ? 0 : 1;
        Debug.Log(isMuted ? "Juego silenciado" : "Sonido activado");
    }

    // Método para mostrar las reglas del juego.
    public void ShowRules()
    {
        Debug.Log("Cargando las reglas del juego");
        SceneManager.LoadScene("Rules"); 
    }

    // Método para cerrar sesión o salir del juego.
    public void Logout()
    {
        Debug.Log("Cerrando sesión del usuario");
        PlayerPrefs.DeleteKey("token");
        Context.Instance.AuthToken = null;
        // Cerrar la conexión con el socket
        SceneManager.LoadScene("Login");
        // Aquí puedes añadir código para manejar el cierre de sesión o salir del juego.
    }

    public void ReturnToGame()
    {
        Debug.Log("Regresando al juego");
        SceneManager.LoadScene("Game"); // Asegúrate de que "SceneGame" es el nombre correcto de tu escena de juego.
    }

}
