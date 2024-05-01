using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TMPro;

public class EventsPanel : MonoBehaviour
{
    public GameObject panel;

    public void HidePanel()
    {
        panel.SetActive(false);
    }
}
