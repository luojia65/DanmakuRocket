namespace DanmuDisplayerWin
{
    partial class DanmakuForm
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            this.components = new System.ComponentModel.Container();
            this.Ticker = new System.Windows.Forms.Timer(this.components);
            this.SuspendLayout();
            // 
            // Ticker
            // 
            this.Ticker.Enabled = true;
            this.Ticker.Interval = 10;
            this.Ticker.Tick += new System.EventHandler(this.Ticker_Tick);
            // 
            // DanmakuForm
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(36F, 72F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.AutoSize = true;
            this.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(233)))), ((int)(((byte)(233)))), ((int)(((byte)(233)))));
            this.ClientSize = new System.Drawing.Size(2020, 202);
            this.Font = new System.Drawing.Font("黑体", 27F);
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.Margin = new System.Windows.Forms.Padding(10, 8, 10, 8);
            this.Name = "DanmakuForm";
            this.ShowInTaskbar = false;
            this.Text = "DanmakuForm";
            this.TopMost = true;
            this.Load += new System.EventHandler(this.DanmakuForm_Load);
            this.ResumeLayout(false);

        }

        #endregion

        private System.Windows.Forms.Timer Ticker;
    }
}