using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;


public class prueba : MonoBehaviour
{
    public Sprite[] etapasCrecimiento; // Array de sprites que representan las diferentes etapas de crecimiento
    private SpriteRenderer spriteRenderer;
    private int etapaActual = 0; // Índice de la etapa de crecimiento actual
    private bool crecimientoIniciado = false; // Variable para controlar si el crecimiento ha sido iniciado
    private float tiempoInicio; // Tiempo en el que se inició el crecimiento

    //Referencia al componente de texto mensaje
    public TMP_Text  mensaje;

    void Start()
    {
        spriteRenderer = GetComponent<SpriteRenderer>();
        spriteRenderer.sprite = null; // Ocultar el maíz al inicio
    }

    void Update()
    {
        if (crecimientoIniciado)
        {
            float tiempoTranscurrido = Time.time - tiempoInicio;
            int indiceEtapa = Mathf.FloorToInt(tiempoTranscurrido / 5f * etapasCrecimiento.Length);
            indiceEtapa = Mathf.Clamp(indiceEtapa, 0, etapasCrecimiento.Length - 1);
            spriteRenderer.sprite = etapasCrecimiento[indiceEtapa];
            if (indiceEtapa == etapasCrecimiento.Length - 1)
            {
                crecimientoIniciado = false; // Detener el crecimiento
                //activar objeto mensaje

                mensaje.gameObject.SetActive(true);
            }
        }
    }

    // Método para iniciar el crecimiento del maíz
    public void IniciarCrecimiento()
    {
        if (!crecimientoIniciado)
        {
            tiempoInicio = Time.time;
            etapaActual = 0;
            spriteRenderer.sprite = etapasCrecimiento[0];
            crecimientoIniciado = true;   
        }
    }
}
