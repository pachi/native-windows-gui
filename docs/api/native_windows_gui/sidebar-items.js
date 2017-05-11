initSidebarItems({"enum":[["Error","Error class that regroup errors generated by NWG"],["Event","Events name definition"],["EventArgs","Events arguments definition. If an event do not have arguments, EventArgs::None is passed."],["SystemError","Error class that regroups errors generated by the system"]],"fn":[["dispatch_events","Dispatch the messages waiting the the system message queue to the associated Uis. This includes NWG custom messages."],["error_message","Display a simple error message box. The message box has for style `MessageButtons::Ok` and `MessageIcons::Error` ."],["exit","Send a WM_QUIT to the system queue. Breaks the `dispatch_events` loop."],["fatal_message","Display a message box and then panic. The message box has for style `MessageButtons::Ok` and `MessageIcons::Error` ."],["message","Create an application wide message box"],["simple_message","Display a simple message box. The message box has for style `MessageButtons::Ok` and `MessageIcons::Info` ."]],"macro":[["nwg_button","Sane defaults for the Button control. Requires a parent."],["nwg_canvas","Sane defaults for the Canvas control. Requires a parent."],["nwg_checkbox","Sane defaults for the CheckBox control. Requires a parent."],["nwg_combobox","Sane defaults for the Combobox control. Requires a parent."],["nwg_datepicker","Sane defaults for the DatePicker control. Requires a parent."],["nwg_filedialog","Sane defaults for the FileDialog control."],["nwg_font","Sane defaults for the Font resource."],["nwg_get","Return controls from the Ui. Panics if one of the controls could not be retrieved.  "],["nwg_get_mut","Return controls from the Ui. Panics if one of the controls could not be retrieved.  "],["nwg_groupbox","Sane defaults for the GroupBox control. Requires a parent."],["nwg_label","Sane defaults for the Label control. Requires a parent."],["nwg_listbox","Sane defaults for the ListBox control. Requires a parent."],["nwg_menu","Sane defaults for the Menu control. Requires a window parent."],["nwg_menuitem","Sane defaults for the MenuItem control. Requires a menu parent."],["nwg_progressbar","Sane defaults for the ProgressBar control. Requires a parent."],["nwg_radiobutton","Sane defaults for the RadioButton control. Requires a parent."],["nwg_separator","Sane defaults for the Separator control. Requires a menu parent."],["nwg_template","Generates a function that initialize the content of a UI. "],["nwg_textbox","Sane defaults for the TextBox control. Requires a parent."],["nwg_textinput","Sane defaults for the TextInput control. Requires a parent."],["nwg_timer","Sane defaults for the Timer control."],["nwg_window","Sane defaults for the Window control."]],"mod":[["constants","Controls constants"],["custom","Custom control creation resources"],["templates","Macro based template system."]],"struct":[["Button","A standard button"],["ButtonT","A template that creates a standard button"],["Canvas","A blank control that can be painted to"],["CanvasRenderer","Object that offers a light wrapper over the D2D1 api."],["CanvasT","A template that creates a canvas"],["CheckBox","A standard checkbox"],["CheckBoxT","A template that creates a standard checkbox"],["ComboBox","A combobox control"],["ComboBoxT","Template that creates a combobox control"],["DatePicker","A simple date picker control. Do not handle timezone. It is recomended to use a specialized date time library to use with this control."],["DatePickerT","A template that creates a standard date picker (dtp)  "],["FileDialog","A file dialog control"],["FileDialogT","A template that creates a standard file dialog"],["Font","A font resource"],["FontT","A template that can create a font resource"],["GroupBox","A groupbox"],["GroupBoxT","A template that creates a standard groupbox"],["Label","A standard label"],["LabelT","A template that creates a standard label"],["ListBox","A listbox control"],["ListBoxT","Template that creates a listbox control"],["Menu","A menu control"],["MenuItem","A menu item control"],["MenuItemT","A template to create menuitems"],["MenuT","A template to create menu controls"],["ProgressBar","A standard progress bar"],["ProgressBarT","A template that creates a progress bar"],["RadioButton","A standard radio button"],["RadioButtonT","A template that creates a standard radio button"],["Separator","A separator control."],["SeparatorT","A menu item separator."],["TextBox","A multi line textinput control"],["TextBoxT","A template that creates a multi line textinput control"],["TextInput","A single line textinput control"],["TextInputT","A template that creates a single line textinput control"],["Timer","A timer control"],["TimerT","A template that creates a timer. Note that because the timer callbacks must be added AFTER its creation, it cannot start automatically."],["Ui","Object that manage the GUI elements"],["Window","A window control."],["WindowT","A template that will create a window."]],"type":[["EventCallback","The function signature for the event callback"]]});