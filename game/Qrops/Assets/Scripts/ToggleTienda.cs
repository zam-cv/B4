using UnityEngine;
/* 
 * Esta clase se encarga de mostrar y ocultar la tienda.
 */
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
