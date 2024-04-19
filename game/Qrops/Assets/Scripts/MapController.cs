using UnityEngine;

public class DragToMove : MonoBehaviour
{
    public float speed = 0.1f;  // Velocidad de movimiento ajustable
    public float minX = -10f;   // Límite mínimo en X
    public float maxX = 10f;    // Límite máximo en X
    public float minY = -5f;    // Límite mínimo en Y
    public float maxY = 5f;     // Límite máximo en Y
    public float minZ = -10f;   // Límite mínimo en Z
    public float maxZ = 10f;    // Límite máximo en Z

    private Vector3 lastMousePosition;

    void Update()
    {
        if (Input.GetMouseButtonDown(0))
        {
            lastMousePosition = Input.mousePosition;
        }

        if (Input.GetMouseButton(0))
        {
            Vector3 delta = Input.mousePosition - lastMousePosition;
            Vector3 move = new Vector3(-delta.x * speed, -delta.y * speed, 0);

            // Aplica el movimiento
            transform.Translate(move, Space.World);

            // Guarda la posición actual del mouse para el próximo frame
            lastMousePosition = Input.mousePosition;

            // Aplica los límites a la posición
            Vector3 clampedPosition = transform.position;
            clampedPosition.x = Mathf.Clamp(clampedPosition.x, minX, maxX);
            clampedPosition.y = Mathf.Clamp(clampedPosition.y, minY, maxY);
            clampedPosition.z = Mathf.Clamp(clampedPosition.z, minZ, maxZ);
            transform.position = clampedPosition;
        }
    }
}
