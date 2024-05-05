using UnityEngine;
/* 
 * Esta clase se encarga de reproducir la música de fondo en todas las escenas.
 */
public class MusicaFondo : MonoBehaviour
{
    // Esta función se llama cuando se inicia el juego y se asegura de que la música de fondo no se detenga al cambiar de escena
    void Awake()
    {
        DontDestroyOnLoad(gameObject);  // Hace que este objeto no se destruya al cargar una nueva escena
    }
}
