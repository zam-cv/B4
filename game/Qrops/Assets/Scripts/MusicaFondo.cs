using UnityEngine;

public class MusicaFondo : MonoBehaviour
{
    void Awake()
    {
        DontDestroyOnLoad(gameObject);  // Hace que este objeto no se destruya al cargar una nueva escena
    }
}
