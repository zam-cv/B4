using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TMPro;

public class EventsPanel : MonoBehaviour
{
    public GameObject panel;
    GameObject scoreObject;
    TMP_Text scoreObjectText;
    
    void Start()
    {
        panel.SetActive(false);
    }

    public void TogglePanel()
    {
        panel.SetActive(!panel.activeSelf);
    }
}
