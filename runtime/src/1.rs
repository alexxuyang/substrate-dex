struct Tokens<T: Trait>(
    self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
);
impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > for Tokens < T >
 { 
    type Query = Option < Token < T :: Hash , T :: Balance > > ; 
    type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ;
     
    # [ doc = r" Get the prefix key in storage." ] 
    fn prefix ( ) -> & 'static [ u8 ] { "token Tokens" . as_bytes ( ) } 

    # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] 
    fn key_for ( x : & T :: Hash ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > 
    { 
        let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > > :: prefix ( ) . to_vec ( ) ;
        self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ;
        key 
    } 
    
    # [ doc = r" Load the value associated with the given key from the map." ] 
    fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , storage : & S ) -> Self :: Query 
    
    {
        let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) 
    } 
    
    # [ doc = r" Take the value, reading and removing it." ]
    fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , storage : & S ) -> Self :: Query 
    { 
        let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > > :: key_for ( key ) ;
        storage . take ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) 
    } 
    
    # [ doc = r" Mutate the value under a key" ] 
    fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , f : F , storage : & S ) -> R 
    { 
        let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > > :: get ( key , storage ) ; 
        let ret = f ( & mut val ) ; 
        match val 
        { 
            Some ( ref val ) => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > > :: insert ( key , & val , storage ) , 
            None => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > > :: remove ( key , storage ) , 
        } ; 
        
        ret 
    } 
}
