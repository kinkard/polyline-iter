use criterion::{Criterion, black_box, criterion_group, criterion_main};
use polyline_iter::{PolylineIter, encode};

const SHORT_POLYLINE5: &str = "angrIk~inAgwDybH";
const MEDIUM_POLYLINE6: &str = "ue`aiBu}nxWFlA~B|c@`AzR~@jMpEgBaAwLsIilAMiBK}BqDu|@QuESyD[gIOyDgEmiAiDuYW_Qq@uQuCa_@{@iFiEcXk@mDa@_CcCcOi@wEoAiLaAoSo@ySiDmx@gD{y@aGwrAoJsvBWgIiAo`@AaTGyLBoHDiD`AyAn@{BT_C?kCk@mEo@{AgAcGUsCKiCYkOWwUi@_wAbA{a@d@eLt@mM|@}J`B_P`AuHdAmItA{Kn@sCzMil@`DwNzF{X|I}m@vDc\\p@iFXwB|AiJHoDAsAEmBKcBWuBaAsEmBaJuAuGgAgGk@{F_@cFSiFKoF?yFLwM`@aT|G}uCAuIK}BCo@O_EQoCM_AWiBaAsDsB}FkB_FwCwG_DqHyGiOqc@ibA{Q}_@qS{d@{K{WmIeSuJsWcM{]_JaYyWay@yj@ceBgc@yoAsVor@kUsp@aUwo@mSip@wYgfAa[ejAsIk[_GgTqFaRkG_TcXmbAuIu]oKqd@oGkYeGsXoEsS_FqU{Jqe@iMwk@uJw`@sMsh@}b@{aBm`@a|Aaa@}yAcTmx@qQqu@gNkm@}Jue@uPwv@yZyuAqg@k_CyOgq@yHe`@cIuc@cN{u@_Lwn@aQc`AsIgc@qP_}@oE}UeQ{|@cWmnA_[urAkXihAyZigAo[_fAibAswCkd@qjAskAouCsgC}mG{iAesCkrAebDee@kjA_Qyc@{X}r@qk@guAah@soA_Rid@uWqq@}}@y_CuMqZqYku@kXus@eTok@qu@soBimA_eD{T}l@{Ryh@av@srBoj@_yAgw@wtBqIiUuyAw|DeV{o@stActDoxAydE{pAotDug@uyAia@akAgb@kmAmd@srAiiAqdD}lA}lD_hAqaDms@ktBeXoy@wSyp@mVmy@aRip@e{@y`Da}@meDqXw`AkZibAyLm_@u`@}mAyq@qoBsf@mrAyg@gqAyp@o~Ayw@ofB_n@krAkp@}vAuv@mbBgjAqdCcs@{{As{CyuGygA}`CcYam@ybAaxBgWmu@gJkUqMw\\wTqk@kSuh@eJ_VmEkLeAoCg@uAc@oAm@gBU_A_@iA]}@e@_Ac@o@g@m@c@_@WUg@Yo@Uc@Mm@Gq@Cu@Hm@L_@L]Nm@b@c@`@k@p@Y^[b@Wf@Uh@Wn@Uv@Uv@U|@}@tEa@tBc@lB[pAm@~Bs@jCy@vCgApDqArDeApCmBlEsBxDwA`CqArB{ApBiBtBwAnAoAt@kAf@_AR}@LiA@{AEyCMiDwAgEwCsEsEuDyE_DuFkD_JcDuL}DkTmB_L}DyUmC_PwCePaDeM}FkQcHoNqMeTgMoOiPuMsKwHuH_EiBaA}Ao@{EyBo@Qsa@qPo[yMkYyKe[sL_UwHeQ}GaScHqE}A{DwA{HuCiBo@eYeKmQeFm_@{Ke{@mNef@iCa[gAwPOi~B_DmFa@uFg@kFi@gT}Dgb@kJmw@yQkGqAc}@eSmJuB}h@uLoB_@}FqAyDy@w@xJ_@hIIdBkAbVaC`\\}@zKQ`C}B|OiIh^wDxL}DbLs^xaAaYdv@iAxCwD~JyBpH_Pdg@_HhTg]daAkGlOaBrDeOz\\}Tbd@ep@jpAuH~M_MvSoEtHyCfEwLzPmG|IeAz@sU`ReN`GoQfFyTfA}TX}T|@kDSaCw@{BoAwC{Bi@kAu@iAiBcF_ByEmA{D_AuDgCaOMi@eAmGwCqJqCyQmA}PmBma@mEwgAaDc{@c@gMuAiWqEebAaAqTAiHMcDIaCCkBJcBNwBLw@Dq@@w@C{@Iw@Mk@Sk@c@_DSsBWoCg@gFw@sHoAi\\c@mTUwUGmYRib@\\aN`@sJx@cO@_DGaCKkAYiA{@cBo@y@u@c@}BIcDRsHx@{@H_E`@_Gp@aGj@mJdAqHx@aP|AqCVyId@gEDaE?sIJqHByLDcGEyXHyFBmE@mHXwE?";
const LONG_POLYLINE6: &str = "zqkm^fqw|`CdFAlAXbBlAfAvAlAxAtA`CJdHb@fR@pG`EEhg@o@pBCfEGnr@cAlAAnFIdDEzzBcDvE_BlFIdSa@tIQnYi@hC?jCFx@jAjAv@vA\\dDjGpAj@t@f@tC|CdRhSpy@h_Add@|g@|OhRfBtCw@j@m@p@e@z@g@lBItBJjAXfAbAjB`BnArBj@]bEeBtTaB`YAnGl@~FvCxKhCzI~@tCn@pEb@nKMjEwE`]uCpOSpGhAhF`BjF~AdCjDxBpYnKhAxD?dHkAtEkDfN{BzGiBvMg@dRUrPC`LbE|VoDxBuMtTwa@~x@}Yzm@kW~h@q@tA]hBeBhEgAdCgG|L}E`K{BrE}@pAo@`AgCtDyVpd@mZrn@iLzVqJfSiFdKsVbf@mCtDgCzBaF~Ci\\bRqMbLwErEyAxA_@lAC\\@\\f@zAmBxBeGxH_DlDo]be@aJjM_F~IiDhGcApCq@`B}BxGw@fD}E`PqDnKmAhEgStk@mBfGuUrx@o@jHiFtOeFrNqGxMyJlOqIxJmPlPuNtN{_@j\\iHfGyIzGmKxGiNjFsYbKww@dVyMvFaGxC_JzFiNvLg]bc@sSrTmJrGcKlE}XlJgmA`]csAv`@}MxDqQ|IyK~J_IxHuBjCyD|GgAlBuXvk@sJlRqE|FeKpJ}aCbsBiEnC}YfQkYvHoLrB}Mh@}Sq@ePuAuRaDqqBa\\aPcCkO{@opByAij@Tig@rAcn@zDwb@rDoWvDe^lFwGt@qL\\yIi@mL{A{JgCo[eNy`@kLai@yOuRkGmMkIwG{GiKaMiGoMuAcGmBiL_Lop@wUekAoHaXgFwNiFoLkF{IsGyH}HkHoGsEoHqD_HsBaFgAiFe@qGKgHPeFl@gIvAoGhBeF`CoExCeEjDwF`HkChEaD`HoC`J{A|Km@hKHfHZtFhBzJ`ChJxEdLvIhLftAlgApErDxHtGfJ|K|EpHhC~E~GlRdDxIpGnTjJhXlGnSdHhQfEtI~FdIrGlG`OvKtY|Mbk@`SpN|FjNnIbRfMxa@j\\zQjLrSnJzLdEfWhFfCZnSdC||@vKxMnB~KlD|LrGvKbIjKpKtG`J~FfOr{@dzBnGdNvJ`ObIhIbJhHzFnCzK~CvN~BfRx@bOM`N}@dM{BnKyCvs@yWpNgExSgGzJaBzJk@nJQlJd@tnAhS`StB|TlBv^tCtiD~Txf@hCzKFnI]nMcBhIeB|HwBtw@w]dI}C~PeGjG{AbHm@rG]`HC|GTdJ`AjJdBpJdCfKtDtGbDdIjF~F~EnGzGtFrH~EzIrDzItCjJzBvK|AvNb@~LA~LWrMaUltD_AxT[lWLtKtAnQvBhLnDfNvN~[tc@~x@pFbJzEbH`GnHfLhLjOrLnKdHlLbHnMtFfRdGvPzDhObCrZbCrMZvNGbPUjG[~Ie@bTyC~UsFnOsEjTyJjUaO~eBulA~PuO|MwNlI{Mtf@s_A|IaP`IeL|GyI|IiKlc@{a@rp@mi@jJuGfLmFrGkBpHmApH_ArmAsFvWoCjk@wKda@}HdKkB~Nc@dN]~u@|AlM^dKb@|Iv@dJrAn\\lGJBdE~@b}@tTv_AxUjCl@xw@vQvLnCxq@bQ|FxAfw@bQ|ShEta@dKfx@|Qhb@nJdGtA~@RnBb@jBb@~EjAfNdDlnBne@dcCjj@z@R`ATvA`mA\\dWz@tl@FbE`BfjAj@~_@t@zm@tBbeBk@`Hw@lEwAnDgApBnAlDl`@`c@pTnRtM`LpCbCpb@b`@rCjCtBhBlZvWRNnFxEduAbpAhBbBtg@`e@tH`HXfBr@zAhAjAxAp@bBVfABtEhEvBlB`u@vp@tS`R~KzJtMpLrj@`g@lCbCdo@jk@zk@fh@jBbBbq@bm@pQ`P`[xXpWrUzWxU~o@bl@dB|AzDdDfy@vt@t^f\\tOhNhZbXpWpUrWtU`s@lo@jHfDjQvP`H`Dli@jh@l\\pQzp@~j@hUp[dQhQxJnI~KdFrI~HbHjGhVhThLfK|_@l^bj@zg@fOjMxOxNn`@t\\t\\jY~hAnbAbVxUv\\l[fo@~j@vPjO`dAx~@`w@zr@nv@fr@nBfBxUdTdWfU~LxKtJpIpYzVxc@|_@bf@db@~n@tj@nZ~XhYdW`FjEhDzCtr@|n@x`An{@zf@jc@~TzSdc@x`@zNbNfTzSpb@n_@jEzD|GdGp]~Znd@ja@vAlArh@vd@nq@`n@~n@`k@jn@|i@tjA~dA|]p[lYlWnKnJrH|Gv`@d^n@h@vI|HjGvFlJpIjo@rk@nKpJfHpGtDfD|[vY`DrClJrIrr@rn@v\\nZtk@hh@jp@|l@vk@ng@fo@~m@rD~Cfm@hl@hCxE`CbCzElE`OhNjgBv~Ah{C~oCjzAvsAjIrHvp@vk@daCjwBdfFztE`HfGnFvFfgBl_Bj}GzgGdoHfxGr`@`^rbBd{A`}Lp{KdyCpmCfr@jn@r~AlwAreAh`A`eC~zBx`C~vB~_ChvBj~@ry@zlCdbCzqBfiBddDvwCfdCfzB~{A~tAbaKbdJfMdLzLvKty@hu@hl@jh@`_C|sBn_Azy@byIncIns@dp@EfBVdBp@vAhAfAxAj@`BLbBMrr@xr@bC~B|kArkArj@nk@rl@ri@na@r^t\\vYlFdBlCvBtAdAxE|DwSte@mHtSkLxh@yLth@iIdZyKx\\mGjn@EfEzE|j@dFha@fGhj@hEb`@bGnd@xDhXdBjQiPjUoChCcD~B}z@ff@_MdGuEtBmD|AwIlDg`@fLqRnF_FpAuHvAeQdDwiDnf@}kAzOyF`AseBbN_KlB_CzCA~CdK`[`gAvkD`mAlyDzRtf@px@hfAjTtd@fEqGlEgG|a@se@`D_EbHaGvJChLr^rGx^x@|MnAt`@`@dUfC~]pH`g@vBjJtJxg@hGzUxNf_@tIfVbXtmAhG|o@p_@v|DfPfuBheCbu^rw@|aLt[fgF|Z`zH`k@`sKfxArwW^jI~[vkGb[luFnAlUxVtaEf{AbjYtl@p}K|@~\\|CleAxFjyCrBbgAnCpXjElRpRvj@~CpFjOnWfQ`TfR|NrjJtmG~l@z]fShN|F`DzRhHrOvFrOjExK`BxQnC`q@tJzk@tG`a@nFb^~Gb`@~Jht@~UhKbEfLdGpJ~H`I`LlElNhKxc@bJje@vPxw@dEzKbGrJpJxJzL~GryBhw@b_@vMzG`CdiA~^fv@lU|QfE`MjAd]Nr^l@zQPhTRjRf@nMbCnHjCnHjFpK|NjLhXdOxa@tPjd@hZxv@~IxOdMfMr}ArqAviB~`B|oFxuE|Z`WdPvKtN~HzPlGnWfIrm@|Qnk@lQvm@hSnj@nQtg@rO~`@tLnb@jMjQ|FhJ`FnIbH|JpMtFlKhEtMtH~_@vElUfDhQ`Il]`FbWtLnn@~Idg@dOxv@lDnPhE|M`FpLvFpL`G`LhElI~BvHlD~NfBpOdCrOhClHbDfHhC`EnIzFpPbJdV|LtCdArYxOv[jO`}@|e@|GhDndAdj@t]nTt]nTrtBxsAfxAz~@rzBlyAnQlJhM|EbUpElg@tL|w@hR`P~B|Nr@bhAq@b\\l@xG`@vP~DfOhHdOdIzSrNpOvHbHxBdHt@p`@zFda@`Gv_@vI~]pN~xAfm@xYdPhj@l^`MtJz~@zy@p~@b|@t_@r\\dN~L|N~JnJzFxmAl`@t|@h[tz@rU`l@pNhrFfyA`HxAl_@bIt_@hI|SfDvg@rGzxDdi@bOdCfKdCbLrFbLpI`yFlqEhKlKnHdM|DnOvBlO|MzlBhBhObFjPjIjMbIzI~JpLzf@xf@~I|J~H`PdY|n@lNtRhIrHhItHzkNdrKzKlKv~@paAvKtJvKbIzOvHzNvF|}EhjBl^~Lfc@~LfmAhXvwAfXps@hP|PzDvJ|@vIAl[sEnLC~OvCn}@dk@v\\jXjZ~Xp`@jR`_@rNdFtDnCvGlEr[jDfXjIzTbKlIfq@p\\ht@fZnYrQtRnIhG\\`Gs@zXcJll@aQzOy@xOxEfK~ElF`IhElLjBnPrE~a@tIdt@|I~q@hTx~AtCvNpH|XxEtP~EfJfZfXbOxPpJ|M`NnZbLfLbRjG|QhKvCrFhBvHNfPyEhoAq@rPfAbYfCrPfDrNxPxd@`NnZfRda@pSfa@~Xdg@x]tz@lh@xlAx_AtwB|Z~u@bMzg@xH~X|D|UpH|}@hQnxBhEdVdHh\\nSpkAxBn_@nAdY|Djj@bGxn@`CnOz@~P_@lHeAzDeBhCuCxBwCnAwEX}WUqKpCwH|FeClEgSbf@wHxTeC|S~@tMxIrWnLbWjFpO~CfOzAnY|@~WaAjX_DnIiDlFwMrFg\\lGum@|GaJlBeGdDiCzH?`IdCjKjJhLvPhO`QfRnPl\\xH``@hJz_AfCvt@y@rWyLb{@oB`HeBzDgE|DkCdBaGlBsFd@}WMqHNcFz@gEjBeCnC{B`Fc@~FfAbMpFvNjOpf@vIpPjKxS~JjLtTnRfd@x]xBxEv@rDSnEo@zDeDn_@oCzWtAfMnBdIjEvGzDrDn[xLf`AxUzM`KxI`MxGvNlFnYbr@h_FlNhv@n}BveJjAfRe@`RaKzpAwJfvAqCxgA@n_@|Anb@vDpj@pFph@lBhc@cAxpBO`W`CrTxHjPdNbYxO`WtI|OrL~RjVtb@vK~MvWpTbKfJbJlPz^xr@~GxO~GtInr@~k@~LpH`JpExt@`Xjd@fR`lB`_ArQlMjNrSf`@t{@|DpJlFpMlP~_@vMle@tDn\\x@n]eEflA}GblAqG`hAyB`d@_Hzb@mdC`bIcFzTcCzTi|Ax`ZO~[dCvWrCpPbGtQrM|W~[zu@`BtD|fB~`E`kGpsNp[ht@~_@p|@lWtl@|Wnm@zWlm@r_@|{@r_@|{@nJhRpKlOhSzShMjMpSjLpQfHd~@nVtpAx\\zsHhmB~`@`QtXlNleFjqD~yBtiAv~KfuF~DzAdGzBrnJz`DruH~gCdUfJfVzH`RdDzOvAnRv@~b@tCzj@xBrf@jC|f@fEzXrEh\\`FfNpBnYvGp]xHlSrFpX~IhXpJdPhHvL~EjQjJd[|NlSjJ~WbNd\\nQ~OjI~W|MxSjJpM`GvTzJ`WdNxObN`MrOlI`PnH|RrHn_@vAte@jAvSdDhx@rFrwAv@bRfAvMlAbQb@fPR|Ll@jLtBpr@~AxRpAxQvDxS~DbRlEvPfFvNzGzN`FjJdGnJ|FxH~HvIrFbGlgGbaG|\\d[`T|Qb_@hY`^`WxQ`Kzx@jf@xg@jXbf@lXp]dQdOlKrTtR~SbV~`AvpAzQxWtOv[nFhTxBtSb@xP?fi@Qvp@p@pTnCtP~HfWvGfLnEfFpK~IjQvMlMzIt`@~WbTxNvQrLjaAzq@baAhp@zqAd~@xe@t[h]lVba@tXnDtBfrAd_Ajw@dh@xu@ng@nFrD`hBhoA`iCbgB`jD|_CnTdO|bBtgAz_Bb`Avt@`c@nT~Mt\\vSrw@rf@hj@h]xo@ne@ht@ln@fqA`jAlp@|j@lb@ta@t`@l]r^zZb\\p[jWxZtVt]hVv`@nNnWtY~t@dGfUbLrb@bR~z@~Qvu@tXn~@vk@vaBdUzp@~Vrq@fDbJ|Nnb@fGlQxKt[~Kx\\dExLhGjQvG|NbOzVzoE|fGdUdVpNhMlO`NhQ~KhFbDjO|HlM`FvVvH|QtE~ObE~UvGjSjIzUvM`NrItK~GlV`QjWpTde@v_@vBlBtXvRb\\rWnSnUdQ|RpWd^v`@vt@fs@~fBlZrq@p]dx@dQpi@vMza@~GvShHxU|`@nqAxVrv@lTvp@bWju@~d@~oArvIlsRtrH`iPhDlHhhMbvXbUlf@r_AluBtOpZjGlQfCrHpLld@t@rCtI`_@lWplAbIrb@jNxp@tOpt@zQ|z@|ChLt@vLpAlU~AbVtG~s@dCpRpHtYbGjl@rFfYrFli@lL|~@lMr|@jUt{AjOpcArR`rApCzRdDno@dDzURrANlAlAtI~B`LnEjQtCxHrC`N~Jrq@hIbp@vMb`A`ZprBl_@bhC|jBbgMbr@~}Ent@d_Fjs@l`FfiCtqQfa@rlCzVzlBhIxeAlExs@~Cn~@~Ff|FlBzxDj@heA|@haCiAreAy@~[_BjYcCff@yDhi@cF|n@a_@x{DcDdu@uBfp@i@`j@Ebd@n@ln@~E~rA~V~tD~d@nkHpm@niNngBfrb@zp@~|O|PdaEhExgAxCtu@fAb{@f@ldBd@htGr@t~KxAxgTv@nyLf@frFIhm@_@zpDbFxlYPl~@|AtzSMzcJ|B|sCb@nzBQvo@dAvSbCnMpEtRpDdJxJjPxI~LxQjPbSxKpXvKvf@dR|g@nQhUhIlRhIff@nUre@hWl`@zW`a@j\\pb@p`@bhAv`Apb@nb@hZv\\dXr^pSd]bWpd@rOhc@lNpd@rPhm@xP~`ArVnwA|[dsB`Ql{ArQlcBdQjkBhCpZtLtqA|kC~qYfB`R|Gbu@zu@llIhGje@tLji@|Uhs@ntExoLruD|uJzn@~yA|Zrl@zZ|g@boCdsDvxBzuClt@jhAr]zk@r]pp@x\\fp@vg@nfAhwAjbDdu@`cBdgDrwHbgDlwHlrAd_DhoAlmChjApnChd@tdApr@n_BxEvKdSje@d_AtsBzgFjqLtvChwGx}Cd{GfTlf@zpPrh_@tvH`cQfjCn_Ghq@n_BhN|^`Lnb@nYnlAfI`]pQ`i@bSpc@fX`c@`|@hoAnbIlfLjs@hkArUf_@jSbe@vh@hsA~~IdnYbQ~n@`Qrs@vrApjGfz@llD`Nzf@zKz_@vYb|@lpSrpj@rq@dhBllB|jFxPfj@tOlr@lJrh@lJzp@jp@`cH|k@lyHzIv|@rGd`@`C`KfOps@|[x`AtYjl@fP~\\l@|@`i@xy@td@dp@rRt\\tJnQfNfZde@nwAzPnf@v`@znAh}@llCdZbiAdY`|AtX|hBpqA`fJffC~nQ|xA~fKfoA~sItsDvuUf{BztNbVlsAjUru@rGjQ`\\xp@v^jk@df@|g@lfGh|EfhAlz@`t@|h@pk@p[paAz\\tgCny@vIbDvOnEpNlDzhC~h@pkAn\\nkAx\\|JvDnHnExElEvGdGpHpH`FxEhExCbErB`GnA|JlAbWdD~Jr@fPGbWs@`KIfFTvEv@~DtBdGlD`KlGrSdK~ZxKf|Abb@nPpEhMpCjM|AjQ^d[C`ZV`g@dAhVb@vOrCtHfBlMpCjXjDvPvC~T~G~MjFrRlIfS~HdOnJdUfPd}@de@jo@fe@`s@pe@`MjI~MxHbWtJh}@r[zg@tXp`@vUdNnMf\\fX|}AjiA~S~LxIzIfD`FfBtGhA~GfAvFpBnEbElDzDrB`DzAtEpAbCz@pApAnArBjA`CrDtE|E~DxInGha@`YxHdHpGlIvItN~c@bn@`C~CjAxD`@zDAbEkBdF}HfRgD~K{@jP`@pO~B|KnMhYpNh\\jIpUpFtYjB|Jh@|HC`Ji@`SKlFb@hFjA`EvBlF~DfFxDnH|EzMnH`LnD`KdCdKzClN`XzsAnEbRdI`XtO|d@zJnZxJv\\vPpm@rHxRvIbNjKfPvGdL~DzL~BtN|BbOnOxxAfBfOjBrHlBvFhCdFbC|GdBvGtEvPrGb_@pMhs@fLlx@~Pr~@nh@rpBzInc@fBvRhEve@rB~MfD`L|FrOhH`NpEnKvL`XjDdJ~CzK~AvElBhEbDjEtE|DnQlMtD|CzBjCnBzEx@tG`@vLmAf_@mAtZuBni@{@hHyAvDmCvCiD~B{H|DsErCiEnEmB|CuBlIqAvIiCrRO~DHhGr@rFvAbGdLtXtTvw@x\\rmAfJzTjEvGxEjKxCnK~GhVpAbEtBrDhDzDtD`E|BjDpCjGfDzM`HtU~FtPbOzW~NdXxw@r}A|PhYfGdNxEpN`D`SzCtT~c@bgDxEfWd@~Bf@zA^t@r@n@fAV|@EjASfBkAlA{Aba@eo@vQu_@~PoY|RsU`NqOfKwMlDaHtBkGb@wFH_I?yDPwCXiA`@k@n@]v@M~@Br@\\~AbBrA~B|BfHrA|Hv@nHb@vJe@dP}AtRwFfa@gCvXwAt`@z@b_@rDl\\jBlUl@rT`A`TrG~eAxBlSh@~PxAbVh@vT{@~RS`Se@xQFfJLrNNhL_@dGmA`HwChIsFfNgDdMiBzLeA|Ou@tQd@hXn@bw@_@`p@@z]l@tOjBrT|DxXpDpWjFzZ|Nrk@|Hb_@pDvRbB`QrA`RjD~XjDjQjEnOdFxYzDt[zBtTf@dODbc@Kv[j@h\\`BfVvF`^zEt^|K`p@`Nrq@tKjb@rIbZpKlX~W`dAh_@n{A`Nre@dLxb@xIpb@|CvRnAvU|Cvc@pFpmAfE`t@d@h]`@va@q@dg@kBnXgA~WSrUqCbYeMnqAaGrr@aA`d@PtVvEfgAx@lZIpT_Ddv@qCfpAh@rlBf@bQt@dQlBtOtMbl@nPxk@zCpQdAzQn@hi@jGdwAdBdv@RzWaAp[mCb`@i@bSEjRl@lSzCrr@bAfn@bAlJ|DzZlBnVb@rXg@pc@C`InApT`Fb{An@dOzDzwAxBt]dC~XtBtTfCbLfOh_@xAhGf@vEh@fOf@bOjAnKnAhS~AzXtDd`@rArQxC|XjDtSpShmAnGhc@vCbUp@fPInNIxRZbMx@dQxA|[f@lXl@rQKhOJxJXtHdCxPpHnTrFbNtHxPvBhNrGz_@lJxXjHbW`DtNxAfJhA~JzHnf@zAtJnGnWrGrQtM|ZpKja@bHd]nGx`@tDpQbOdq@bKle@vDp\\fEpq@lAjRbAfNxIzl@RfGEhDe@hCiAvAsN|HeHrGeC|FmBlHsNb`AoF~^eDnV}BbOu@~ICzKXdV~Bvd@jAr`@A|HwAfH}HpQaBzIi@jGn@rGbBlQdCdQBtI~@vH`@vLi@fOoAzRgBbWW~OIdVBt}@a@tUoC~d@wEtk@mHtiAmAf`@_Cvr@mBr`@_BnRiAbSoAxPkBzTpP~B`A|DdA|Hn@xMIzRe@hJcBhIiEdRkKjs@}Ijt@yQt{BcAzOoCdNyGtU{EbLiCfMWhEFbFVrL~B~XvC~X~@tYGv[yB|[wI~v@}Jdj@}Kpz@uJlg@wUby@sPzh@iCxJ}AxJy@~JeJlbAyJxw@s^laBoObp@}K|q@ov@piEg[daBe^v~AmIn`@eG|b@_D`r@}B|q@wLrfDoGdcAsAfa@Abb@fGteAfIffAhN|tBfBbR~CxMxGnOnQ`WjStZlQbTlK|J~`Afo@vE|C|C`DrBxFv@~H@zIiBv_@wYrzCQxPn@nXzGx\\`G~PzXrx@dWvl@zQz[|VpVbYnVb_Ajk@j}@rf@`d@rUtUhLlShGrMfHrMjL`K`JlLtKnh@`_@hVxTpdBrkBpNzNlJbIjHxEvNxHxLrJlXhZrLbPdJjPf]vs@n_@~cAvXre@nq@fhAph@br@rJxR|IvWdLlYpM|Vtj@t}@ze@f_Az\\jo@jNnWzErLdEjJpFdItQxSfQ`Sre@z]hE~ClL|KtLtNjJtL`EpGdD`IxDfJdGxL~ChFrEhG|@hBX|AQbCe@|AyAxDwAnD{Rj_@{OdZ_@bBJxAt@t@~AdAli@nWfZ`P|[nRv@z@a@vA_JpLoEdHa@tCC`Cp@bCdCpCjBfBpBlEdCnItI~Ydd@zgBfD~JhD~E~D~D~IdHdOfMdBjChAjDVvGYzPRnVpAdStA|MdCpMxFxSxFvQvChL~DzJ~GvMbElL`GbU~BnS~BzUf@l]g@h\\HrRv@bOh@jOIrOaBtRkDpZcJhn@ab@laDehA`gHsk@zvD}Eh_@o@fL\\nFxAhFhD`ElM~K`h@rb@|m@li@nl@`h@pEdFfBdCvAnEdFdSbCnItCfJrFvOjBhHZzES|D}@zCcCvEcHzLkKnNyI|NcL|TsFxPY`Fb@tFrFr^hCrUZdQZtKjDpGnIrBpOmBxHpBlQ`Qbo@lr@jBlMoA|K{@dLoOff@aCxObCzPhInTxe@d{@vJ~LzNzKht@xj@|\\|VbPdOzFzM~ApJ`Dr[jCdb@VhKf@lB|At@nOtC`KfDvq@zYd{@``@f|@|\\frA`k@duAxp@ddAni@veAxo@hNtJrIvDrGt@`]jBfHrAdHtCtEzFdCpKvFdM~G|ElErB|G|A~K_BhIQhIf@dNhCtUdFx^pKfzAjj@vvBzr@vJtFrCnErCnHtt@n`CxoAlhEnCfMa@|HoEjMymAhqE_@fBOnBNdBr@rAhAh@|A@zBe@pIyFfp@gc@bMaFnWaFxn@cHfoBmN|uI}r@vHiBxFmEnHeEl^_Np\\oKb_@kGjV_C|WgAxw@nAl`A}A`gAm@|XxAliAzHn^gAvW}D`K{BbEE|CrA~@nBn@lDWfGqClIgDnJSjE^hEdBfCxBx@`ELhe@@jNFbGT|]zDrL`@tK`BrIzBvs@xTb]hMrQ~Hfn@zc@~Z~QdTtI|OpDrOpBtOfApO`@~e@TtWlApg@pAr]UfTyBxWeGvZmEfaB_\\~y@}IzZiCtZ`BfR[xH_EnIaEdb@uf@lFiIdRwGba@cMfd@wMfm@sOzc@oMnLsBxKbAnMxGhF~FbQfQz`@lYbPnKrNtOnDzNhEjMjGpEtK|A`OxArM`CxJ|DhIpGpMnEf]|I|YxKlTrKnJxN`K`M~NpOjGlOtArPpBzTtEdQ|D|PfHpQnCvMo@pQuAlRn@tOhCbQlDjVlJvX|GzMpFpLpGfIfIpDtOxEzNhHnEfErGfGlGjNbDpJpKrHvSjMxJjO|d@pm@z_@pg@l}@leAj_A`oArJvItA|AjBtBbGnGnIzHfDlG~H|N`A~Az@|AtNpQZl@Jb@Dz@?v@i@hHDdBf@`B`A|AbDdEjDnEfAvA~JhMzHtIrN`NvL|N~DfKdBhId@fL|QfqDrBzPpExMl~AlfE|E|PtCpU@d\\fAvI~Ofi@rUjz@lCdP`NvfB`E~K|e@`cBxGfZvp@hfDtAhQfFfkAtFzaCfC`^~FhQroDpeHzDnHxHfJpFzDlOzIpaInkC`_D`aAljFdaBnRnHdKbCbzAlQdt@fFph@?lPZfoBp@z_E{D`wId@lmCGhqCc@`rCt@pbB@jk@p@pw@tAn\\z@~i@zBb~@jJjo@nJfN|CdNpHlwBbxA`uA~kAvlAxlAhe@xl@hZ~]`FvD~ElBtMrAhNk@tLsDxR_K`QyWdk@{d@p_Aet@`PaKtDaC|EaCnb@aPrSqIfBYjBDnCTvBv@lBjAbBlB|@hDArEYtCk@pBcPnZkR|_@sQv\\mBjEoA|EQtCl@lDlBbDlBnAnBdAxBTzDMvEq@rz@}Qvl@oMbEkAhCo@tCUxBCrCXdD|ArBtBpAzFLrFcAxJ_AzG[xF@|DnAnE~B|CfE~Bl]fMdh@vPdaAj]`mAx^npAnb@tX`JjGxBtDt@rDZvDPdFMfE_@bFmA`FmB`GcFvO_Tbh@ou@fFyHfEwElDkDzDqCfFuCfUyJth@gV|FkChd@}QjKmEdHuCfFeCbEyCfCiChBgEf@yECgGgA_JmMi|@k@oIYsIJmH\\{Fh@yGrA{G~C{MzRwo@lv@i|BzbAe_Dl`@}pAbQwh@hFaNdFwKfFyIrF{HdHyIlE_FbIeGfI{FlrAum@zfBsw@~jCskAxsCmpAneD{{AlSeJnEqAtC_@hEB`EVlCj@lEjChBrAtBrB`CbCpGdIlF~DzD`BrDj@bDG~BMtCq@rBw@|A{AvEsG~HkMxd@{u@bOmTtKgKbL}H~rA{f@r|@e\\jMoFfIkFdGaGxFgHvUo^dgAkgBzf@cx@jTg^|G}HjFyFjIiGne@yXnuB}sAznCcgBrlCwfBbt@sc@hr@gd@nQqKtSiKzOcHbP}Gvy@_[fYmKptAqg@`rAwe@~VuJxC_@tDCbFn@|ElCbDnE|DjKbHhWbAhH\\`FUpD_AnE{CrFqF~Ky@lEShEt@xDxA`ClB`BbDz@lDLtEaAtEiDzVyWlCiDzA}D|@}EhCaVx@iExAkFrDyGxFiJdO}T~Xya@zFwElCcApFa@dIO|GStD_@zD_AfDgAzVyMl]}Oh`@uO|EyBhC}A`DiBbBoAnDwCxCuC`DuD|CiFp]g{@dC_FrGkIlFaEfFaChDgAtE{@xEc@fOo@pFO`G]lIkAfSeEf]aKvEcBlFmCjG}EvGyG`w@weAnCwEzIcWvDyIrAaCvBmCnl@im@jH_EpE}AfHgB~FeAjPeBpTwBhLgAjHaBxGuChD_ChEqCxD_EzEiHhb@cv@|R{\\nSwYtY}\\rWcXhl@_f@ljCmsBflA{`Azr@ej@f`Akv@xHcJxEwJzo@enBfNyYlGyJxGgHzI_I|P_L`ReJbuB_s@~|@{ZtgAk^dqBaq@jmB}p@lcBmk@`l@uQxEq@rFk@xHF~J~@|HhDjF`E~CjFhDhJbBpJh@dKLfKt@z\\PpFf@vD`B`G`H~Hjy@~o@~BzCzCzEpCjGnHhPpEpItBlCjBl@rCf@hBJhEg@tCqAlA}A`AeEJkEa@}D_G{QaCuHmFyO}DqK_CgI[eFR}Dz@yDpCkD~gAg}@vGgHvDyHnMsi@jCeGlCcC~Dm@vDJfDnArDfCt@zA^rC?nDi@|Cs]h}@o@vELrEp@xBnA|BxApAnChApCPfCMbCm@vCwAv_@m\\zbAq}@xOqPlF}JtAgF`AsIr@oOlB_XhAaFlB}BzBsAhDu@dCBnEbAdC|C~@bE\\lF@zMAvMGh[~@rPrAbDfClBjDp@pDN~D}@vEmDtJcVrD}IfB}Hb@cI@}Fy@_G{CyMiHiQyFgPkD}Ke@sEl@mFjDgHfRqYjDoEpEaDxD}@vED|s@rFbSq@hOk@jMe@`HUbMHbJdC`GzFrAzE^pIkDzPk_@xuAwBvLNbLvAlPfK|k@bB~GlCfDzBnAlCl@|CBxBg@vCqBnAkCj@gAlEqKtGwO|IgKfHg@dE|@dCtBvAhCzFnW`Pvn@tl@vcBrCvMM~Kq@pDiAfC_FxFmP~NgFpEcFfE_GzCgHnAqm@TwJx@iNtEsFlDgFlHmCxJuAfNGzGxAjIpClI`HtOdFrJ~FjMhE`IhE|BdBJzCMdE{BzCqEl@{Gs@mEeGeLkG}L{A}FKmENiDx@wCbA{BpBgCzCqB|CgA`Ei@hDDtJbBb}@pZ`^pPxUrPvR`RrP~TpYlf@paBjtCx~AjpClD~MxBlRjBdUbD|^|BhIhDvGtEfFxJhIpqAdf@|JXzHcBbFmCdQeNpKgM`A}DVcDKkEiAyEwB{EqM_TsCkG}AaHo@qHBuInAqI~L}Z~k@_tAd[qo@jF}JpFaJ~LyN|ToRdPgMbJaCtJx@~CtAlCnCvC|D~ZvgAxJjVtHzOtIfP`ExDtFnAnFKdD]dDyAxC}CtCeGZ_Fe@kGmCaKsCmKiA{HBgIpI{`AvMmp@zGa[pDoH~EuGzHaHlLgGn[qLh_@mOjy@oZfnBeu@jEgBfk@k[bl@cTzPqFdJwCryBke@xZwL~J{DtNwFhPmIlN{Ip~AkpAvTs\\f{B_|GlE_\\}A}`@Ui_@hBeO|AaGpCiHbHmKd`Bc|AnhAoaA~PqPfHcFdIwCtKoCfNqChPmEbK_DlJmEfIsFfZqWlX}UhU_Pze@qYpqJyyEbdc@qqTjLqE~GuAlOeBpKV~Gz@vFfAnD|AbL`JdJtKbIdNvrA|kClg@zcA|d@h`A`IzN|GzKlLbPtsDpgFhuA`nB~j@ty@`^|l@~NnWvLzW`_@~x@l_ArcB|s@lnAt[xf@p\\fg@dfBpwC|x@ddAz\\na@zKxM~JhIvO|JbQzKxKdJ|DxGxDpK`HfOlEdI`IbJ`HrInYjX~aAb~@xkBdxAnd@p_@lS|MvTrOpu@nd@xbA|l@lbD|nB`aC~sAvx@hc@rTpI~oB|{@zn@bZpx@nd@lItEprB``Ab_@bRjfApuAdh@r`@|aThlNfiDffA|G`DnJjHtJvJp\\t`@|TtXtIjIdJjH`PtH`}@jXxaAz^teBfs@xhB|i@vfEbnA`LtFfLfJvF`KvEnKbe@||BdE~JbGfJjdAb_AhOvKhhBtp@|ElBrFnB`QjBpRlAnNfA`SbDl}@bUftGdgBrKjDnMpFztCttB|FxF|GvIxK`Uh[n{@nDrHfHrJpNnMzGzHfTv]`PhVbGzGnKxH|OrKxh@pZ~k@d]dZfQlc@nWt`QdsKdzDjhCtMvJpG`G~CpF\\nCo@tNqFxZkdAn}DwBvIkB|I}A~IsAnJeApJw@rJyNdpB{@~LoA|L_BxLeBfKsBbKaC~JsB`KgCxJwCtJgDjJsCxG{CtGoAhCwAbCqCfE}CzDkDnDwD`DmZfWcsArdAg_Avs@eFlE}EvEsE`FgElFkEhG{DtGoD~GwCnGkCvGaCzGwB`HaElNwDpNeC`KyBdKkBhKuAfJiAjJ_AjJmT`yBKhCyRnpBwD|YkH`QkQjQih@nf@yIjM{Whk@eMb]oUdw@wDfVyCjSm@zI_@~IQpICpILb\\f@rKdA`PlA|J~^lvB|@dDjA|CxAvCfBjCtB`C~BrBhCdBrCxAxCfAjWrHdD|@`DhA|CtAtC|AhCbB`CjBzBpBtfDh}C`CvApClAhD~AbE~ApD~@dFx@bqEfZphJto@`XVfKQhMsAhVeEb[cC~p@}B`Iq@lJyD`LiH~GeCbH_BfGPxJtCxd@nStL~EhG`BhIlAjJl@vn@lBvJf@~Er@nItCnJtF~h@~_@vSxPx]xTxJrF`M`FvhCv`Apx@h]f_@lPpIpEzEjArGjA`HT~Hm@vOiCtzAo[zKuBfG{@~IK|Ip@pNlDdrCx`AraAz\\lfAp_@zk@zR~UpJlG`DjEhDxBpB~CpDrDdFpEpHvFfK~D~InDvIpq@|eCpiA~~Dvp@~`ClBnHdBpHdB`JxAfJdg@hxDbYfsBna@ziC|OdfAjCdPdC~KfCxJtCtJdy@rhCjn@fpBvDvLxB|Id@~C\\`DR`DJ|DA|DK|DedBrhh@yGdpBOtLAtLLtLX|Kh@|Kt@xKfT|dC|Dja@hApTx@nMXr[BpAn@rr@D`FlAxn@@vWCdh@LjMHtIx@tFlAtFhC~FnK`LhLrKdPrOn[bZjU~RpYfS~d@d\\nSpMxNxLzFxEdCzDpAnEfArGfB`IfG~\\hKjn@tK~l@jDdR~Pr}@xn@dnD`r@rvDjg@doCbV~qAhCfLdBtIzBtInBfGbCpHxDdHdD`FlDpEdHzF|iBttAx|AriAzjAf|@z]xWlcCliBj`Abs@xaAdu@|wA`hAjIdLrH|NvIfYvDb^QpVmCt]sH`^_}ArlDoa@t~@qNjVuRbYawDl}DgVz]qR``@y|BtrHa@lM~@rKnExKnG~HbMfHb|BxfAlNxKxKxMjNhStOhb@vPnj@nCta@KhO{BjO{ItYycBltDeNtVePlOuZzTqb@jVefAfg@okCjtA{KzLaIvNiIva@{IpoAkGtgAu@bM{@dLy@fJaBdI_CfHeE~IoEhGqJpHyQjMewBxmAq]r[aO`Uqg@|wBcAdO|B|LxExH`ItFzFxAnHe@dKqDrIqIxCkKhEk{ArCaNjGsMnGcGzGeDbRiE|`AiNvWkBhmA{@hKz@hJ`DfK`I~G~Plo@b}ChRff@zcC~}EvJvLhJpFxK|Cbc@`AxbA[pRoE|MiIh_AioArGsFfHyDdNiCjOLjNxDpJrIjHhLhCtPnG~kAhEvKjIbFxIrAhoAo@nPXpJhDrEhF`E`Lh`A~hG~CbKtE~GlMrI|u@h^jCnAlI^pIcAxHiE~GaK|UmwAfDkJhCwBtCmBpD_AtEAvEp@tDxB|DtDhnFt`RrLrZnErZdFlaAjBlfAwDdqD_JtqDFvQjAbHrApEdClF~CpDzEzCpiCbsAxFbBjF\\fFUjCk@`EiD|EoFrtAazCjF}H`DwDrEqChEiAvF[jGEn{Dfj@xbB~Wnp@bKji@pGhLdBhItCfHbEdH~Gn_@no@l^xs@pp@ftA`EpIdCpId@pGL`TtB~|Ej@fMbDfRnE`M|ErJnK~Ln{@dx@bpAfqAfF|DfFzBzEh@~E?lFm@lD{ArD{D~FsMrf@{`ChD_JrG}FnK}DbMaBvL`Ab\\`GfaId{@pN~DfEpFtBtG`AtIyA~JwGjMotDfwEawAdiBy`A|kAcTt[{LdNeQxK_YrN{b@nPg_Ada@qThN_n@bh@{LdIuWnSgd@~]uF`FuDjFq@nFRdDjCtC~ExBlGCdFgA~n@m_@tJ}ErLCpMtClTxIjTdNrR~R`PbUh[lu@vLjZzJt\\jBpNPlLaBj\\cDz[}Zt{Am_@jzAuA`OLxN~B~^j^p|B`GnZbdAjcCnpBp`FpHzO|JnLxRbQlSbO|X|KnKvIhHvNtI|LvS~HpRlFzMb@vUaBt\\wKbIkAzG`ArEpDxNjTxOnRbJvFzNvFzm@|Ilg@xC|[xD`h@fNr`@jV`PjJrGdJ|FdKtCxL`CfSxD|b@fCnN`EvHbGbFnJfFrTxEvFk@`GsC~F}IxTsm@~B}KEwLcBsMwQ__@cSys@mCcQKyHz@yIjE_M~j@}|@dGqGzFgCnIuAfJy@`GoBhLsGzOwMfZ}SjYsMjY}K~w@kZl[aI`SqJ`VkNnd@uSrIsGnG_JdH_PpKaIbNqHjJwCjPyCnImCrEmDxNyPfO{PnIkGxHcD`~@yUvO{B|ZqBl~AQz~CfSx[aHbu@wKphAIteByB`JlA|HlFrLhMjWzVvJhGdOvEpR|Bru@rEtJ`EbTbQj`@pWzYrKlrAhn@~f@vR~IhFrI`Nv@dE`@~EoBlKcP|P{FrIgEbK_J|_@uJ`[aO~ZuInOcM`PgLnHmWnK_z@j_@_UpHgS|E}CtBaDzDcBhFa@lI`@fKdCdJnE|Jpa@vh@za@vf@h_@r_@hMrIbInBrHP~Fo@~BiB`Oe[jFmGb`@mV`VuM|ZcFxc@cDbWgEjKKhIh@vGdCrLpJlW`[zGtLnM~_@hFdLhIpLvVhS`LnMbm@zrA~FpJ`DhBxEl@fHg@lHcD~dAuhAdJaI`BcAdB{@|CmAbD{@hDk@jD[lDIlDHjDVhDj@bDz@|CjAnj@xX~CdAbDz@pEz@rEl@vE\\xEJxEAvEQvEc@rEq@nEaAla@_HjK_AdGDnFjCbErE|GjRnFvLnFjHtJzFfL|EzjB`]~n@b\\xeB|gA|h@xg@dwCj|C`DlIlAtID|GmAjNkKfs@g@fGv@tt@g@tJoUrsAsApJ?hFrAxG`a@ppAlElIfDjCtFpAl`AbDr[VhMo@hQ}ClSoHxqCqtAdJ}ClH]zDJlG~BxF~EnlAbdBbHrGfHjCpH?|OuBhr@iQvOiH`KiKlHmMnC_J|@sK}@mP_a@kyBq@aPd@yKlDmK~EwJjG}ElLaC|b@{A~FaC~EuDtYye@pGmG~E{Br}@{QvKEhMz@zy@xOlMtDxHfExsAxmBdR~TrIlGfOdCdMb@x\\_A|KPrI|CtFzHxB~MnElqA~BbOpD|JdFdH|DrBlE\\lHeAx}@qb@puAks@bHwAhQx@~EjCxFnFjJxN|ErDhEhBdGJjGy@dFsEdfAwgB`GsNfAoHfHkiAdIsdClAcKnCgIjDaGjGyEbG{AzE_@tEE~G~@jGvBjJxHdEhH|ExKnd@||@jI`IbGdF|GnBrL~AjIDfKe@xJiBbpBacAvKwFzI_DnIk@lDIjF~@`H`HvE`HfDzHhC~Gp`BpkElIfRbKrLdEzCrGrCnFjA`J`BxINpI]~HkAdH{AtIaExRmNbi@ub@piAq~@tNaJ|JyD`FuA`HcAbKk@dJ^tHf@lJlBjJbD|GpDtIzGnFdGvF~HdFfLhAbEnA`G`BfO|F|w@`NdpB~AbRtBfMvCvLdD|IpDdHfF|IbrA`bB`D`I`BxIj@nKOpG{@fGmBlHiKj[u@tE]~CUtEAzEl@zFpAjFrC|GnEjFpDvCzCzAfEbBvFnAlEt@zJh@v|@I|dAMdIDhENtFVxGn@rIjAnnAlTrf@vIhPxCbLpDlHxDrHnFtF|GvDbGvBlFhD|OxTzxAxA|FzBlG`CbGrDzFrGjIhsAd}ApFhIdDrHlDnLpApHhA|HzCvUzBlLlCnHnD~GnD~DvDlDtFxCjIlCjXpGv^pHdJ`FvIvHtEhJtC|GbBtI~AbJdCpr@d@~p@?`Gq@jJ_FvPyIhMaG|E{KtF{d@vPy[r]aWv]gLfLaH~BkJjAwt@`GoJhBoJzDwRjNsb@jd@mu@|s@kCtHY~CBpEzBxKnFvH~k@lg@tMpSnN`a@jRd|@`KdWbJ|J`l@va@pZhe@zNbOpNpLts@r_@vGrI`EfMhEnc@?~MsDl]eCvPaDtFmEnDqGvCaLfC{R|GiMbI}ChFee@zrAyNvXw^fg@iBvFE|GfA|AnBVzCcBndBo_CpKaKvGyEfLsDvp@uOzKeFrHqGdFiMfIa]pCoQ?sIkC_u@Ei~AlAoHvDmEpJ{Dhs@gIxHUxI~@fLvGvcAz{@ds@jj@l^`P~JvCjKb@rLc@pKiB~SiLlh@}b@jb@g]nO}CbI^~E`ArGlDlEhHjCpg@rClf@bBlo@fC`s@rJlbCzBvp@dEb|@pDhUtJhXrQ~b@fQv_@~N|Z~JlWrBpG|@|OxB|NxF~P~HvLlW`WtJtFrZnG~K~D`t@b[fXdHtV`DjeAzPpLhAx^h@`Y{AtDl@hCfBtA`DvZfcCrGb\\|CrIbLlQhB`Er@`DCrBcA~BaCd@{GkBgSuRoH{IgJ_Qs[ytAkDwJ_CqCmDmAkFBwI|@su@~PqY|K{[~O_EhAiE\\co@^uMc@oJ_B_H}DmNgPyBwAaDiAoFSoGpAsAzAc@bCNbEfB|DrHdHfPpKpXrMvXhJ~QvCxOC`OsBrv@gQ|S{BxElAtCzEr@nFR~a@tCfTh`@rtAbHdZzFrMvGnIbJjFjHbAze@kAbF`A`F`DbG|BzQvB`F|AtBtCpAnElDb`@lFnM`JnMzUbSlj@b_@j[rWbYzWrSjNhNjLhVpZdj@|m@rRdWpRn[vn@zoAhXhp@lOtc@hMfc@x^naBdLxa@fJ~T`^pf@bKtKnOzLt~@|k@t^jX|XtVlMtN`NxRho@tq@rTbN~HpCzGZrFW|EoBnFiGfh@ugAtRkf@fV{p@`Jk\\nPmy@xEak@Sed@eAoTiCsXs@cWBq]tAc\\d`@q`C`EgJtDcEdCy@bF[nCN`IfI|l@jt@tlAvtAvKjIvNtGbSjFdUjCfUh@tC~AxAfBxDv_@lDjQfP`g@xDtH|JlJdN|G~e@~MdUhKlMzE|j@xOblAhc@x]hRfFvF~BnEl@bFNdIW~O}B~n@cB|UmCxTqEbXkHlYaa@f_A_EbNkBdNn@z\\vd@d}BjFxMlF|HpKxDvGvAlOdAlKSvRcB|KgB|LKtO`AdUdC|VtE`LdDrKtGnDjEvBfGlBpNV|KyBdo@HvHn@lGbJj[dCvMfIpkAlA`DrIjI~RlLlSpHpWbF~]`DvO\\jHu@je@aNhh@iIjZaDhPyClTsI|WeHbPgC`NcAzO[|OZxL|@nHnApNrF`i@zPrk@lOvo@tYvjAlk@hi@hRzFlD|DzDtTl^nRfTrXtR`M`G`QdDvNdAxWFh`@eDbYwFbLmDdJwEfI}G~FcI`EqKxBqLpDglAo@{IyBeKoGyO}J_PeYk[gIqGmg@c\\oFoBul@wM{GaAc[RiDSmBsA}AuGCkBp@wCbB_CxBwAtCy@~FGzF`ArRrInGrB~n@~HtNbFbb@pSr[nLrKtGti@xl@rGpJbDxHvAlGpBhr@pBrMpDjMfJtQhErMtBzLxDzj@|BxTNjGsBtNuBnFeChDsE|B_JjC{c@xHui@pRizA`\\_QpEuJ~@yFCcFm@o^gMkO_BuNB_v@~BwPvAyFBaJkBcKoFaf@mh@wHoB}LYegArLwIBsHaA{FwBcGwEqDmGiL_[uC{C}D}B_FmA_SBkRnEm_@bGaOxGkOxNuF~DgHzBsHb@uVGqJi@qFPkHx@kt@fNiPt@qx@CgHq@sx@aOkm@}Dsr@gIaI_CiWaLwGkAuDOgIt@yi@jMkGp@ko@tAgy@xFoWOwRqC_UiHqVcM}ReNoJcJwFuHiFuJ_IsSkEgG}DoBkHsAaF`@aDzAqCxEuSvl@mF|GcGpCsFRaGW_G_BiEeDaCaEaAgFuLc_BoAqGuCyC}CaAiECsEb@}CzAoA~AgFxSaa@|fAik@xuAcGrT}D~[]lUzDpa@hOtjAlNd~@NjI{@|HuAdGyClGyDxD}F`DmaAr_@aGh@wF?mFaAcFwBqEeDmTyYeLuKuVoSyEoFaCmGyByKgBqNiAm^iBwEsCmDmCcBwDGaDx@qCzBwAzBmA`Gs@rPkI`e@qChGyL`IcFt@uEWkGoB{GyDmM}Jc^w[}J}FmDeAyCCuEx@mClAeBvBW|Cn@tChChDbGdDv\\lKpKzEzGbFfU|]bHnIvGhD|^`HvGxDhDvEvAjFV|G[|GoH~W}D|J[lGZdI|A|Ghc@vkArXbk@zGzHrKrI`MnFrQrEzm@`ItM\\ff@uCvc@Ll^bDpYvD~FMzF}@zKuG~EwGrIwWdN{VlV{WdJyH`GiDvEkA|D?|CjAdC`Ej@rFsA~EgEdJkIxLiCzHwAjIa@lLtBhj@|Bjb@pBbUpF`Ltg@pt@nPpMdNbFxFbApHFhEy@jJmJxLcGlLaKbGsJfGuNzAeJxAqEpB{B~B{AnEB`ErB|ApChCtK}Hdi@iGlU[xI|Dvv@z@rDuBb`Ac@nl@dBhPrTnhAdInT|IxJhN~Kd_@rO~IjI~CnOnAtXc@tQkCfGeDjD}A|AuCh@wEi@qE{Ck[q`@cI}MuN__@wOcg@yWkl@qDuLiCwVnA{WeAmLwKwHoJ~B_EnFeA|FjAbJhYniAnPzv@nI|Y|Txh@vp@`nAxDhD~EjBhF^zHWza@sM|VuGvFe@`Fd@`EjB|ChEp@~Dy@vF{BnFgGxJqeA~jAsElGwBlHGzHj@zFhBpG`D~ExDlCpEx@rc@?pEj@jDnAnBrCl@xC}@fJuCzEg^|\\_FhGuBnFc@dGCbGxMtjAzPvpAnGfj@|BfJpCbEdFrC`KlCpOrCre@x@vGh@nE`D`D~E~@rERnFc@jFyAnFiEjIaN`RyKlRiC|Hg@dHJbIxApH~B|FvDbFdExCff@vI|C`BpBbEhArFNnFi@~EqNtj@wA~IElG`AfMrApHvEhLnGzLr@`DJ~IwAxG{D`KoQdLcCxC}@hCo@dEKxGpCve@m@vEaDtEqDpCeFhAcQ~HwL|OoAnHjFvPfIxRlVjc@vIzVlWph@`BjF^tGObFmD|OcAzLlChn@fRjfBCxHqCjZZhL`CjMbn@`eBjKlT|Yzs@@lKaC`EcQrCcH}@kB}AeAmGGoSqC}ZiBqGuEuOwSwa@mC{HgPg\\oGeO_Hs^kAsJUkQwD{i@{Hkv@wEwScRin@gHaPyImVoHcQmRu]qc@gj@aGeLoA{SvAgP_AuMc@yAq@qA}@iAgA}@oAq@wAc@yAS}AC{AHyAXwAh@oAx@eAdAy@nAk@xA[`Bl@xFzHv\\hG`RzK|OxLdTfMbXhMzZlP|f@lOjm@|BrVbF`e@~Bjf@`@xdAlAbXpCfRxFfTvOj_@rNdg@xFz_@dDl^`ErUbH`OdPxYzI~UrErXhDvJ`ArErCrItHxLfp@bi@pOjIbSnAdPd@dIpCzH~E~GxQxEdVrOvOb`@|b@tQxXbMpLfTdH~\\xA`LmDxMmEjL}IdIcB`K`@vCtHb@dEgPtc@UlGb@jGdB~B|A`CzCtBlDtAvCp@bI[`YeH|N_ChKw@|HAzCDhFh@tEL`EhAzFnC`CfEp@pEy@jEwCxDcEp@qGIeMsD{IsBiHiAkFYwKvCa_@jJgQ|CiF{@iI_@yJaEsDeFoBkCmNqTaJsGqNuAsGH_EQcFy@mFuAkN_AkPqJi`@g[kW{ZcOsVcFeOgB_FcF{HaEcJcJsCeJ|C_GhFpBtN|JpKnp@hs@~b@rg@pZdXf]rY`Y`ZbRbNhU~KtTbN`l@f_@nD|BtErDbNhKtOnMjE`F~PtUzJhLxFbFdT`PrTnMbF~FpBlKWpMPlHxA|GzClFfPdFhJtBhFj@rEPxPd@dGzAtGn@zFnAzEjBjF|BrClBpFxCpJjC~Gd@zEq@jJmDrPuLlGiAnJA`GnBdDvCjCrFTdDExGmAlI_A|G}@lKL|FlA~ExAzDtCzD`KdKxLtKbFhEtE`EdD|BhIpFdDtBnJnFhHvD~F|CdKfFxLbG~FfCzIxDtKtEpElBdGvBjH~CzFjDpEhDzBhDx@jBbAxF^zDRxEn@jORlFdAlLdAlExBfEpDdFnHxIh[l^~DvE~CxE~AbDxAbDtAbFt@`Gn@bFVxBdAfLj@hH\\tFd@bFnAhElCpFnCpCbBhAnA|@hCl@vDh@fHo@bEeBpCiB`DiCpDgFrCuHjDgJfG}InHgFhI}AnGz@zDbB`ChCdBdEl@pHUpMs@`Q}@lQgArRXhLjBnO~OnYvLjTdDzKvCvEbFjThCh^pQlp@nM`^~GvOlBlMzB`OrBtNjGhPbGpFjP|JhP|EdRvHjY~DvO}@`SoA`b@?|\\sE`VuIfZiJxLwBlIbBlG|EvCfKQdMcAjNqB`OjAnRnCbM`GtJbJjCzL|CzM~ApNzDlLdGdLhJxFhCrIhAjIOnHm@|RmB`IG`IjBpHzGxEvNj@`IkAzKaC~N?hMnDnM|IzJxLdF|OjGrIhMhAdJm@hMkLng@q@hJdB|KzCnFzGlE`GvAhUzBlKxBnFzCrDhFpBbHAjKkA`GwFbLsPb[qMdUgLhRsEbO{AjWe@bYeBlj@FbRxCfKzCbEbQfQjC~DdBfFrCxQbCzIjDtE`GnD`T|GpIlDxF~ErDzGnLf]bG`OdFzFbH`CpHt@xNRbJGfIlAtH|CnFpCzLfHrGfC~Gx@~Fc@dQ}D|HmA~GKrFRnG~BjF|F|CrIh@rImB`RmApJJtI`DtIhIfIbPvJrHpIvBrDbA~GC|a@\\dNrCdKbFbG|FrBtGNdIyA~e@yPjPmGlKw@bHbBbG`H`EtKtDrO`DzHvEvEvGtEbRvLdFlKbA`LgApL}BhLcA~I^|IxE~HzHnGbPzLvFrGvAfI~@zHjErSnDfKfEzGbGtEjP~NvD|GxFvPlDfIrFhIrGtHpVjV~IjJ~GdFrJxCpJx@pKt@tGbDtCdHCxIqDlGcHjA{Hu@}R{FeJaFoHsGwIeKuJiJsIuEqOaEsUcEaKkCiNgGyKaIoMoToJ{OgFmH}FgDeHcAaGTqFhC_D~GOnKnAxP~Dv]pFxLrHnH~JrFlMnFne@`OpSjHvTdOnUxS`MbJtPrGlu@tRlD`FhAjIyAbGmEbDiI^qIiC{QyK}McHwy@oYyJoAuJ?eH`DoC|H^bIvEzIdJpLlCzGi@xIeEfFkGnBsH{@uFsDqOoRiGiDsIJaGnD{AdHbA|HxExFzq@b^hJzBjJe@|FkCnLsKdHuGnJeEvJEfJjBfGfEvRjNvIhEbIv@`KmBpJyGvQkQxIgIjI_EfI_AxI~@jI~EdC`IKlNgDtNeEt[hA`ZxCxU~L~y@nWdtAX`SgB~S`BrT|DnMzBtErEvCrGvChLrF|FnDzDvDxAzEB~FeBzFoDtFoFtFmKjJiFtF{DlG}BfIi@vIh@rIrDnWfChMfEhKfF|IfHnHdWhS|N~JtD`EdBfETtFoApF_DpDkF`BiFKoFkAwFeCaYcOiHwHcWse@wUkf@_NuXwC_FwCwBuEk@kEJ_EbAaDlBeCnDuAhFOvG`ArGpBzGpWhp@pVpc@~]rh@xDpEzE`D|FpBfHr@nrAkAvD@bD@tFzApDlCpChEnAvG`@rMdBlKtDlIvIlMfIdNnJfRnN`X|P`ZnRrXhOjR~IzJhOhKl{@nc@xQbKbJhJ`HvMdEtH~p@vr@zMbX`EjDdEnC|KdGfCjDzBdJfCnHpEdGxFpDrIhCxTvD`NnErN|G`N`L~LtKlFrCjEz@xIaAbI{@jEVbDjBzB~CzL`d@`CnEjEpExFxCfFz@fHEhM{CbFs@xDKlCpAvB`DTvDg@jJ?dLP|G~AfI|CzH`N|OvJlT~c@h{@bOfT~FzEpEb@rFQtDeBjE}DxI}UxDuHlCsCxCgAnDEfCz@tBxCf@zC`Cjb@xCvZ`Ktm@zBhSpBhH|CtHpE`FzDzCrCt@rLf@fFz@|HfD|HdDfQvDdJ|BhMzE`H|BrIpA|PlApK~A|m@rQzGdDdGxFf]db@bFdElCl@|CDzDeAzJeGfC_BtDaArDDvEfAl{@|j@vM|L`FhCpEfA`FDpEQdGcCbLuE~DaAbDQdDz@~JbKpE|D~CfAdETrFeAvSeJzEgAbD?hCrAhBhElC|ZdBtE`E~D|cAjo@zGlFzBnFr@zEKvDoA|BqCtBuDh@aEP_Ec@oDaCqB}BiMsYmF{HiIgIkt@qd@iGwBaIo@yQc@ujAlAclAcCwd@wDgCPgC`AcAfCIrDNxCnA`DfCbB~Gb@h]gClKWhJ\\`Hr@fg@nKtPlClKz@xOb@dG`A~LnEfF`AlFErI{@nDc@nDb@lCpBpEnJpExIfFjGpShPrCdEjBpGrFvWzBnHhDtEj}@vr@nDrFdBzHtGnf@zBrKzDfIfVvZjKfI|HtElL|DneBd]bDxAjBhEZnEoAfDcD~CkEh@oDEoUcMqHqBq^oEexAiS}CJcDlAeBdA}@nC]bEr@xD~AdDvErA|g@J|Kn@bLhBhk@bSptAzx@dJ|G`FhHvo@ztA|ErNtBlNfE~o@hAjIfCzFbk@`v@xA`DjB`FvNp}@rYpmBtEnXvE`O`NlVzAtE`@lDWnByAxDoLbRqE~LyAhM}@tS`@jLvNlhBjEv\\`YhfAfY`bA`Tjl@dRvb@xGpGxF`AfF?~h@wTjJkG|QkRrFeEtq@yNbFEjElArC~CfAtF[fFmC|DwFpDgfCd}@sIjBwEQ_GaAeVeL}C]gEn@gCjAuBhCm@vDZpEhD|OtG~^jBzS`@jROxS{DnvAZ~QnA`YlFhi@hBnI~DzFlFbDvGfArpAdAdFnAhDnBfY|Vbc@vZ`ExFxA|GhLf_BTbMDd]|@pJlCbMr_@vz@|CrDpEjAvE?vNgFbGkAfEKzEjAbCzCbAlFcFl_B|@dJdB~IdG`K`m@fl@nGjJtDrKlQj}@vE|MpFhJ~sDvaEzJfI|IzEdMvBtOb@hMh@hI|AhGhCz`@z]zz@xg@fH`CbGb@rFwAzDeEvQ{c@|C}DxCyAlCJ`C`A~AfDx@tElChkAbAnMlTx_AbJtXhOdZvKbPvKtKhLpJpSlK`g@`TjStJhOfLnjAzaB~DpDdHdBnFVpHeBn^wWtGuCfEm@dEf@xDhCnAzBDrDyAdG{^no@_GhOoCrP_ClgA^`PfB~LnEjPju@jpAtlAvuBpTl[jc@de@`\\v]nQfWpM`Yh\\heApLx]fHnMhMlRvWvW`pB`mBpi@ti@pQzUxLlVvLf\\bV`t@v\\hcApJhb@vG|e@fUpoBhKp}@ja@|oDj@lG|^~dErG~r@~Et`@xAfTHnT}@baAvBp|@zB|m@jElk@~Gzo@xTxlBtBdSvAnSr@hSVjSIpSm@lUyJtfBg@dOBbOlJxtCnMnuDzA`f@p@zb@w@reAUngATrZBz@l@bYz@hZdAx\\jL`cBlMt`CxAlUzGffAhBrZ\\rFtArRv@zFjD~RlDvNxH~TrJfT|r@lnAnZri@p_@`r@v^dm@l\\rp@xClFx|@h}AfRnXb\\bi@`GhGpUzTfX`Tz]dSnf@dX`H~DxIxFbGlFrF|FjEhD`EzEtClFtDvI~DvMlD~LnIj[vJl\\jJp^pFvOfElIrBtDpFhNrFhKlGfK`JbMnIfKjHtIhLpLtWpU`b@f_@bA|@pGvFxo@tk@pUdSj\\fYxL|Jb\\zYbThRxn@zj@lNfMr[dYbQjOnPxNrJdHdUnPl]rTdFxCfSnLfJtF`J`GxIjGhL~I~KlJj_@~\\~MxLlOtM`DnCbQbPtNfMhLlLpD`DzDrCjElCrE~BxEpBjB`A|XpLdXhLdd@tPz{EflBxUzIhaAt^|h@dT|bBnq@|EnB~gBbt@|R`Iro@bWrCfAts@bYziAdd@pa@bPdY~KxRxHrKhEx^vNpq@`X`o@xVp]dNre@bRvcBzo@~{Bl}@|[lMfRrH~eCvaAnfB`s@xRxHxtApi@rk@bU~}Abn@lHtClnD~uAl~@n^h[`Mjw@~ZjsAjh@jzAvk@dRnH|iAdd@~e@jR`t@~XzzA|k@fe@hRrfA`c@znCbeAdo@~WjMlFhBt@jsAhj@`q@|Vxc@rP`Y`Jf_@tLpl@hP`XpHhXpH|WjHjF|AjN`HhUrJ`vAta@hmAn]jGdBx~@jWhkAr\\bo@vPnd@|KhVfHzYbIhX~H|@Vxs@bS|Y|IrDpAlIvC~CpAnJdEnPlH|WnNbZnPbR|J|b@tUzVlNvYvOnDnB`LhGb]jSzA|@hP~J|JnFlB`AfYfObb@dUfb@xT|~@~g@hAn@pR~KdNtHjQ|Jxi@|Y`DfBvr@z_@neA|l@f}@xk@pQvM|OnM|LrKrP|NnRdR|XpYnRpTrDfElH|InIzKxOjSxB`DxTx[tPhVnOlVlAdB|R`ZpLnQ`IbM|HrLtBpD`SxYvNrTlTl^|IlLfEzGfWb`@|LtRlV~^pFlIdI~LlSlZzKzQ`O|TrGzJlUv]T\\LJlAnBrAtBrJ~NdKtO\\j@tc@zq@nFpIpE|GvU`_@zQ`YvIhNlGbIhPvWlD`Gv_@lj@rk@j}@dF`IrRn[xPnX|LnRpG~JrDxFxCxEzKtPfPrV~RpZz\\jh@rDxFzMnSzQzXbCtDjLnQjGnJfF~H`EjG`LtPjI~KpJjM`M`O`GdGpJlJhShSj@d@fTpRvSjRl@l@dPpOzTrSp@n@`[pY|EnE`HpGdMhLlSbS~RfU|EvF|BlCfLxMxD|ErD`EjDjEfDvE|C~EtCdFhC~D~Tba@lY|g@pA|B~BfEhApBx[rk@vCjFhB`D|f@z|@fDbGjLvSxD~GdB|CzY~i@|BnF`ArDTnEYzEIbDt@nG~@tDfBzCjBdCnE~DxCb@lFBfF~AfD|BhDtCth@rl@~^zb@`HxJxG|JjH~NtHjQlL|XvLrZdKtWjd@jhAnAtCrGzVrAbDrGtKhJzKdNvOpHpIhApAjb@ve@|ThWNPrBnC|CdENTnHpIro@`u@zPjS~DnFzD`GrDzGhDxGxFrLdBpDnf@dfAff@~`Al@rBZr@|m@j_B|ApElBrFhBjQbN~qAlGx_@r@rD~@nDvAhEdBbEtBxDfEvFdW`\\lt@v_AaKdIcNlM~[~_@dBpB{BnAe]vRkUtMcDhBqAt@kHbEyaAtj@sAeDiDmIaAoDwAkI";

fn bench_polyline_decode(c: &mut Criterion) {
    // `PolylineIter` has optimized `count()` method, so we should use `fold()` to compare with georust
    c.bench_function("decode_short_polyline5", |b| {
        b.iter(|| {
            black_box(PolylineIter::new(5, black_box(SHORT_POLYLINE5)).fold(0, |acc, _| acc + 1))
        });
    });
    c.bench_function("decode_medium_polyline6", |b| {
        b.iter(|| {
            black_box(PolylineIter::new(6, black_box(MEDIUM_POLYLINE6)).fold(0, |acc, _| acc + 1))
        });
    });
    c.bench_function("decode_long_polyline6", |b| {
        b.iter(|| {
            black_box(PolylineIter::new(6, black_box(LONG_POLYLINE6)).fold(0, |acc, _| acc + 1))
        });
    });

    c.bench_function("decode_count_short_polyline5", |b| {
        b.iter(|| black_box(PolylineIter::new(5, black_box(SHORT_POLYLINE5)).count()));
    });
    c.bench_function("decode_count_medium_polyline6", |b| {
        b.iter(|| black_box(PolylineIter::new(6, black_box(MEDIUM_POLYLINE6)).count()));
    });
    c.bench_function("decode_count_long_polyline6", |b| {
        b.iter(|| black_box(PolylineIter::new(6, black_box(LONG_POLYLINE6)).count()));
    });
}

fn bench_georust_polyline_decode(c: &mut Criterion) {
    c.bench_function("georust_decode_short_polyline5", |b| {
        b.iter(|| {
            black_box(
                polyline::decode_polyline(black_box(SHORT_POLYLINE5), 5)
                    .unwrap()
                    .into_inner()
                    .len(),
            )
        });
    });
    c.bench_function("georust_decode_medium_polyline6", |b| {
        b.iter(|| {
            black_box(
                polyline::decode_polyline(black_box(MEDIUM_POLYLINE6), 6)
                    .unwrap()
                    .into_inner()
                    .len(),
            )
        });
    });
    c.bench_function("georust_decode_long_polyline6", |b| {
        b.iter(|| {
            black_box(
                polyline::decode_polyline(black_box(LONG_POLYLINE6), 6)
                    .unwrap()
                    .into_inner()
                    .len(),
            )
        });
    });
}

fn bench_polyline_encode(c: &mut Criterion) {
    let short_points = PolylineIter::new(5, black_box(SHORT_POLYLINE5)).collect::<Vec<_>>();
    c.bench_function("encode_short_polyline5", |b| {
        b.iter(|| black_box(encode(5, short_points.clone())));
    });
    c.bench_function("encode_short_polyline6", |b| {
        b.iter(|| black_box(encode(6, short_points.clone())));
    });

    let medium_points = PolylineIter::new(5, black_box(MEDIUM_POLYLINE6)).collect::<Vec<_>>();
    c.bench_function("encode_medium_polyline5", |b| {
        b.iter(|| black_box(encode(5, medium_points.clone())));
    });
    c.bench_function("encode_medium_polyline6", |b| {
        b.iter(|| black_box(encode(6, medium_points.clone())));
    });

    let long_points = PolylineIter::new(6, black_box(LONG_POLYLINE6)).collect::<Vec<_>>();
    c.bench_function("encode_long_polyline5", |b| {
        b.iter(|| black_box(encode(5, long_points.clone())));
    });
    c.bench_function("encode_long_polyline6", |b| {
        b.iter(|| black_box(encode(6, long_points.clone())));
    });
}

fn bench_polyline_transcode(c: &mut Criterion) {
    c.bench_function("transcode_medium_polyline6_to_polyline5", |b| {
        b.iter(|| black_box(encode(5, PolylineIter::new(6, black_box(MEDIUM_POLYLINE6)))));
    });
    c.bench_function("transcode_long_polyline6_to_polyline5", |b| {
        b.iter(|| black_box(encode(5, PolylineIter::new(6, black_box(LONG_POLYLINE6)))));
    });
}

fn bench_georust_polyline_transcode(c: &mut Criterion) {
    c.bench_function("georust_transcode_medium_polyline6_to_polyline5", |b| {
        b.iter(|| {
            black_box(polyline::encode_coordinates(
                polyline::decode_polyline(black_box(MEDIUM_POLYLINE6), 6).unwrap(),
                5,
            ))
        });
    });
    c.bench_function("georust_transcode_long_polyline6_to_polyline5", |b| {
        b.iter(|| {
            black_box(polyline::encode_coordinates(
                polyline::decode_polyline(black_box(LONG_POLYLINE6), 6).unwrap(),
                5,
            ))
        });
    });
}

criterion_group!(
    benches,
    bench_polyline_decode,
    bench_georust_polyline_decode,
    bench_polyline_encode,
    bench_polyline_transcode,
    bench_georust_polyline_transcode,
);
criterion_main!(benches);
