using UnityEngine;

public class ToggleTienda : MonoBehaviour
{
    public GameObject canvas; // Referencia al canvas que quieres mostrar/ocultar

    // Función para alternar la visibilidad del canvas
    public void ToggleCanvasVisibility()
    {
        if (canvas != null)
        {
            // Alterna la propiedad de activación del canvas
            canvas.SetActive(!canvas.activeSelf);
        }
    }
}
