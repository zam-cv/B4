using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TMPro;
using System;

public class Utils : MonoBehaviour
{
    public Dictionary<string, CropType> crops = new Dictionary<string, CropType>();
    public static Utils Instance { get; private set; }

    void Start()
    {
        if (Instance == null)
        {
            Instance = this;
        }
    }

    private void Awake()
    {
        if (Instance == null)
        {
            Instance = this;
            DontDestroyOnLoad(gameObject);
        }
        else
        {
            Destroy(gameObject);
        }
    }

    public void SetPlots(List<Plot> plots)
    {
        foreach (Plot plot in plots)
        {
            GameObject plotObject = GameObject.Find("plot" + plot.id);
            GameObject cropObject = plotObject.transform.Find("crop").gameObject;
            GameObject growthObject = plotObject.transform.Find("growth").gameObject;

            CropType cropType = crops[plot.crop_type_id];
            cropObject.GetComponent<SpriteRenderer>().sprite = Resources.Load<Sprite>("crops/" + cropType.name);
            growthObject.GetComponent<SpriteRenderer>().sprite = Resources.Load<Sprite>("growth/" + GetGrowth(plot));
        }
    }

    public void SetState(Player player)
    {
        GameObject scoreObject = GameObject.Find("score");
        TMP_Text scoreText = scoreObject.GetComponent<TMP_Text>();

        GameObject verqorObject = GameObject.Find("verqor");
        TMP_Text verqorText = verqorObject.GetComponent<TMP_Text>();

        GameObject coyoteObject = GameObject.Find("coyote");
        TMP_Text coyoteText = coyoteObject.GetComponent<TMP_Text>();

        GameObject cashObject = GameObject.Find("cash");
        TMP_Text cashText = cashObject.GetComponent<TMP_Text>();
        
        double score = Mathf.RoundToInt(player.current_score * 100.0f);
        if (score < -100) score = -100;
        scoreText.text =  score + "%";
        verqorText.text = player.balance_verqor.ToString();
        coyoteText.text = player.balance_coyote.ToString();
        cashText.text = player.balance_cash.ToString();
    }

    public void SetTopPlayers(List<string> topPlayers)
    {
        GameObject topPlayersObject = GameObject.Find("topPlayers");
        TMP_Text topPlayersText = topPlayersObject.GetComponent<TMP_Text>();

        string topPlayersString = "";
        int i = 1;
        foreach (string player in topPlayers)
        {
            topPlayersString += i++ + ". " + player.Trim() + "\n";
        }

        topPlayersText.text = topPlayersString;
    }

    public void SetCropsTypes(List<CropType> cropsTypes)
    {
        foreach (CropType cropType in cropsTypes)
        {
            crops.Add(cropType.name, cropType);
        }
    }

    public int GetGrowth(Plot plot)
    {
        CropType cropType = crops[plot.crop_type_id];
        var porcentage =  (float)plot.growth / cropType.duration;
        var index = Math.Floor(porcentage * 4);

        if (index == 4)
        {
            return 3;
        }

        return (int)index;
    }
}
