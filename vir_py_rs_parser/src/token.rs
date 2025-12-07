use syn::{custom_punctuation, custom_keyword};

custom_keyword!(def);
custom_keyword!(pass);
custom_keyword!(None);
custom_keyword!(is);
custom_keyword!(del);
custom_keyword!(elif);

// False(false)	await(No async, properly ever)	else	import(not applicable)	pass
// None	break	except(* No exception flow yet)	in	raise(* No exception flow yet)
// True(true)	class(No OOP yet)	finally(* No exception flow yet)	is(Would it ever be used????)	return
// and(&&)	continue	for	lambda(No function yet)	try(* No exception flow yet)
// as	def	from	nonlocal(oops)	while
// assert(hmm)	del	global	not(!)	with(N/A)
// async	elif	if	or(||)	yield
